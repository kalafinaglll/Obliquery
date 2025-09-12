use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tonic::{transport::Server, Request, Response, Status};
use crate::network_lib::{rss3, send_message};
use rss3::rss3_server::{Rss3, Rss3Server};
use rss3::rss3_client::Rss3Client;
use rss3::{Message, Ack};
use std::net::SocketAddr;
mod network_lib;
use network_lib::{start_server};
use obq_backend_rss3::rss3::RSS3_arithmetic;
mod SharesStorage;
use SharesStorage::ShareStorage;
use tokio::time::{sleep, Duration};

//main collector: assign a party to get shares from all parties and reconstruct the secret
async fn reconstruct_secret_test(local_shares: Arc<Mutex<ShareStorage>>, main_collector: u32, protocol:u32,step:u32, party_id:u32, ports:Vec<u16>,addr: String) {
    //retrieve shares from ShareStorage
    println!("Party {} is in reconstruct_secret_test function...", party_id);
    //let mut sent_acks = std::collections::HashSet::new();
    let my_port = ports[party_id as usize];
    let my_addr = format!("{}:{}", addr, my_port);
    let send2port = ports[main_collector as usize];
    let des_addr = format!("{}:{}", addr, send2port);

    println!("get shares, protocol: {}, step: {}, party_id: {}", protocol, step, party_id);

    if party_id != main_collector {
        let mut attempts = 0;
        let max_attempts = 5;

        while attempts < max_attempts {
            println!("attempt {} to send shares from party {}", attempts + 1, main_collector);
            let share4send = {
                let storage = local_shares.lock().await;
                storage.get_share(protocol, step, main_collector)
            };

            println!("success.....");
            let share4send = share4send.expect("No share found to send");

            println!(
                "Party {} preparing to send shares ({},{}) to party {} (attempt {}/{})",
                party_id, share4send.0, share4send.1, main_collector, attempts + 1, max_attempts
            );

            match send_message(&des_addr, my_addr.clone(), party_id as i32, 1, 0, share4send).await {
                Ok(_) => {
                    println!(
                        "Party {} successfully sent shares ({},{}) to party {}",
                        party_id, share4send.0, share4send.1, main_collector
                    );
                    break;
                }
                Err(e) => {
                    eprintln!(
                        "Party {} failed to send shares to {}: {} (attempt {}/{})",
                        party_id, des_addr, e, attempts + 1, max_attempts
                    );
                }
            }


            attempts += 1;
            sleep(Duration::from_secs(1)).await;
        }
    }

    if party_id == main_collector {
        println!("Main collector {} waiting for shares...", main_collector);

        loop {
            {
                let storage = local_shares.lock().await;
                storage.show_shares();

                let share0 = storage.get_share(protocol, step, 0);
                let share1 = storage.get_share(protocol, step, 1);

                if share0.is_some() && share1.is_some() {
                    let rec = RSS3_arithmetic::reconstruct_fromS0andS1(
                        share0.unwrap(),
                        share1.unwrap(),
                    );
                    println!("Reconstructed secret = {}", rec);
                    break;
                }
            }

            sleep(Duration::from_secs(1)).await;
        }
    }
}




async fn receive_shares(addr: String, ports:Vec<u16>, party_id: usize, mut rx: mpsc::UnboundedReceiver<Message>, share_storage: Arc<Mutex<ShareStorage>>) {
    //let mut share_storage = Arc::new(Mutex::new(ShareStorage::new()));
    

    while let Some(msg) = rx.recv().await {
        let mut storage = share_storage.lock().await;
        storage.store_share(msg.protocol as u32, msg.step as u32, msg.from_party_id as u32, (msg.share_1half, msg.share_2half));
        println!("Party {} received shares from {}: ({},{})", party_id, msg.from_party_id, msg.share_1half, msg.share_2half);
        // The lock will be automatically released when it goes out of scope
        
        // Send ACK back
        // let addr = format!("{}:{}", addr, ports[msg.from_party_id as usize]);
        // if let Ok(_) = send_message(&addr, msg.from_ip.clone(),party_id as i32, 1, 1, (0,0)).await {
        //     println!("Party {} sent ACK to {}", party_id, addr);
        //     //storage.show_shares();
        // }
        // println!("--Party {} received ACK from {}", party_id, msg.from_ip);
        storage.show_shares();
    }
}

async fn distribute_shares(addr: String, ports:Vec<u16>, party_id: usize, share_storage: Arc<Mutex<ShareStorage>>) -> bool {
    let secret = 12345;
    let shares = RSS3_arithmetic::share(secret);
    let mut sent_acks = std::collections::HashSet::new();
    let my_addr = format!("{}:{}", addr, ports[party_id]);
    let share_tuple = shares[party_id].clone();
    let mut storage = share_storage.lock().await;
    storage.store_share(1, 0, party_id.try_into().unwrap(), share_tuple.shares);
    

    while party_id == 0 && sent_acks.len() < 2 {
        for (i, port) in ports.iter().enumerate() {
            if i != party_id && !sent_acks.contains(&i) {
                let addr = format!("{}:{}", addr, port);
                println!("the distributor is: Party {}", i);
                let share_tuple = shares[i].clone();
                
                match send_message(&addr, my_addr.clone(), party_id as i32, 1, 0, share_tuple.shares).await {
                    Ok(_) => {
                        println!("Party {} sent shares ({},{}) to party {}", 
                            party_id, share_tuple.shares.0, share_tuple.shares.1, i);
                        sent_acks.insert(i);
                    }
                    Err(e) => {
                        eprintln!("Party {} failed to send shares to {}: {}", 
                            party_id, addr, e);
                    }
                }
            }
        }
    }
    println!("Party {} finished distributing shares.", party_id);
    true
}

fn string2int(s: String) -> i32 {
    s.parse().unwrap_or(0)
}


async fn test_shares(addr: String, ports:Vec<u16>, party_id: usize, mut rx: mpsc::UnboundedReceiver<Message>) {
    let secret = 12345;
    let shares = RSS3_arithmetic::share(secret);
    let mut received_shares = std::collections::HashSet::new();
    let mut sent_acks = std::collections::HashSet::new();
    let mut sent_success = std::collections::HashSet::new();

    let mut share_storage = Arc::new(Mutex::new(ShareStorage::new()));
    let mut storage = share_storage.lock().await;
    let my_addr = format!("{}:{}", addr, ports[party_id]);

    

    while received_shares.len() < 2 || sent_success.len() < 2 {
        // Try sending shares to parties we haven't successfully sent to yet
        for (i, port) in ports.iter().enumerate() {
            if i != party_id && !sent_success.contains(&i) {
                let addr = format!("{}:{}", addr, port);
                println!("i is: {}", i);
                let share_tuple = shares[i].clone();
                
                match send_message(&addr, my_addr.clone(), party_id as i32, 1, 0, share_tuple.shares).await {
                    Ok(_) => {
                        println!("Party {} sent shares ({},{}) to party {}", 
                            party_id, share_tuple.shares.0, share_tuple.shares.1, i);
                        sent_success.insert(i);
                    }
                    Err(e) => {
                        eprintln!("Party {} failed to send shares to {}: {}", 
                            party_id, addr, e);
                    }
                }
            }
            else if i == party_id {
                let share_tuple = shares[i].clone();
                storage.store_share(1, 0, party_id.try_into().unwrap(), share_tuple.shares);
                println!("store my own share: {:?}", share_tuple.shares);
                storage.show_shares();
            }
        }


        // Process received messages
        if let Some(msg) = rx.recv().await {
            if msg.protocol == 1 { // Share protocol
                if msg.step == 0 { // Initial share
                    println!("Party {} received shares from {}: ({},{})", 
                        msg.from_ip, msg.from_party_id, msg.share_1half, msg.share_2half);
                    received_shares.insert(string2int(msg.from_ip.clone()) as usize+msg.from_party_id as usize);


                    storage.store_share(1, 0, msg.from_party_id as u32, (msg.share_1half, msg.share_2half));
                    storage.show_shares();
                    //drop(storage); // Release the lock
                    
                    // Send ACK back
                    let addr = format!("{}:{}", addr, ports[msg.from_party_id as usize]);
                    if let Ok(_) = send_message(&addr, msg.from_ip.clone(),party_id as i32, 1, 1, (0,0)).await {
                        sent_acks.insert(string2int(msg.from_ip) as usize);
                    }
                } else if msg.step == 1 { // ACK received
                    println!("Party {} received ACK from {}", party_id, msg.from_ip);
                    sent_success.insert(string2int(msg.from_ip) as usize);
                }
            }
        }

        println!("received_shares len: {:?}, sent_success len: {:?}", received_shares.len(), sent_success.len());

        // Small delay to prevent busy waiting
        //tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    println!("Start to reconstruct the secret...");
    
    let share0 = storage.get_share(1, 0, 0);
    let share1 = storage.get_share(1, 0, 1);


    let share0 = share0.expect("share0 is None");
    let share1 = share1.expect("share1 is None");
    let rec = RSS3_arithmetic::reconstruct_fromS0andS1(share0, share1);
    println!("Original: {}, Reconstructed: {}", secret, rec);
    assert_eq!(secret, rec);
}

#[tokio::main]
async fn main() {
    // Example setup: 3 parties
    let party_id: usize = std::env::args().nth(1).unwrap().parse().unwrap();
    let ports = vec![5000, 5001, 5002];
    let my_port = ports[party_id];
    let addr = "127.0.0.1".to_string();
    let my_addr = format!("{}:{}", addr, my_port);

    let (tx, mut rx) = mpsc::unbounded_channel();
    //let (tx_shares, mut rx_shares) = mpsc::unbounded_channel();

    // Start gRPC server
    tokio::spawn(start_server(my_port, tx));

    //println!("this is a test...");

    let mut ready_parties = 1;
    while ready_parties < 3 {
        for port in &ports {
            println!("port is: {}", port);
            if *port != my_port {
                let addr = format!("127.0.0.1:{}", port);
                // Try to send "ready" message, ignore errors
            if let Err(e) = send_message(&addr, my_addr.clone(),party_id as i32, 0,0,(0,0)).await {
                eprintln!("Party {} failed to send ready to {}: {}", party_id, addr, e);
                // Do not panic, just log and continue
            } else {
                println!("Party {} sent ready to {}", party_id, addr);
            }
            }
        }

        println!("Party {} waiting for others to be ready...", party_id);

        // Wait for "ready" messages from other parties
        if let Some(msg) = rx.recv().await {
            println!("Received message: {}, step:{}", msg.protocol, msg.step);
            
            if msg.protocol == 0 && msg.step == 0 {
                ready_parties+=1;
                println!("Party {} received from {}: ({},{})", party_id, msg.from_ip, msg.share_1half,msg.share_2half);
            }
        
        }
        println!("Currently, {} parties are ready.", ready_parties);

        // Wait a bit before retrying
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }

    println!("All parties are online. Running protocol...");

    let mut share_storage = Arc::new(Mutex::new(ShareStorage::new()));
    
    let addr_clone_for_receive = addr.clone();
    let ports_clone_for_receive = ports.clone();
    let share_storage_clone = share_storage.clone();
    //let (tx_local_shares, rx_local_shares) = tokio::sync::oneshot::channel();
    let receive_handle = tokio::spawn(async move {
        receive_shares(addr_clone_for_receive, ports_clone_for_receive, party_id, rx, share_storage_clone).await;
        // println!("receive_shares completed, sending storage");  // Add this
        // if let Err(e) = tx_local_shares.send(local_shares_storage) {
        //     eprintln!("Failed to send storage: {:?}", e);
        // }
     });

    let (tx_done, rx_done) = tokio::sync::oneshot::channel::<()>();

    let addr_clone_for_distribute = addr.clone();
    let ports_clone_for_distribute = ports.clone();
    let share_storage_clone_4dis = share_storage.clone();
    let distribute_handle = tokio::spawn(async move {
        let result = distribute_shares(addr_clone_for_distribute, ports_clone_for_distribute, party_id, share_storage_clone_4dis).await;
        let _ = tx_done.send(());  // Signal completion
        result
    });


    let _ = rx_done.await;
    println!("Distribution completed, checking storage...");
    println!("Waiting for local_shares_storage...");
    {
        let storage = share_storage.lock().await;
        storage.show_shares();
    }
  
    
    let reconstruct_handle = tokio::spawn({
    let share_storage = Arc::clone(&share_storage);
    let ports = ports.clone();
    let addr = addr.clone();
    async move {
        reconstruct_secret_test(
                share_storage,
                0,      // main collector
                1,      // protocol
                0,      // step
                party_id as u32,
                ports,
                addr
            ).await;
        }
    });

    tokio::try_join!(
        distribute_handle,
        receive_handle,
        reconstruct_handle,
    ).expect("All tasks should complete successfully");
    
    

    // Retrieve the local_shares_storage from the channel


    //Verify storage contents before reconstruction
    // let storage = Arc::clone(&share_storage).lock().await;
    // storage.show_shares();
    // drop(storage);

    // println!("Spawning reconstruct_handle...");
    // let reconstruct_handle = tokio::spawn(async move {
    //     println!("Starting reconstruct_secret_test...");
    //     reconstruct_secret_test(
    //         Arc::clone(&share_storage),
    //         0,
    //         1,
    //         0,
    //         party_id as u32,
    //         ports.clone(),
    //         addr.clone()
    //     ).await;
    //     println!("reconstruct_secret_test completed");
    // });

    // println!("Waiting for all tasks to complete...");
    // tokio::try_join!(
    //     distribute_handle,
    //     receive_handle,
    //     reconstruct_handle
    // ).expect("All tasks should complete successfully");
}
