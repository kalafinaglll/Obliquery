use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};
use rss3::rss3_server::{Rss3, Rss3Server};
use rss3::rss3_client::Rss3Client;
use rss3::{Message, Ack};
use std::net::SocketAddr;

pub mod rss3 {
    tonic::include_proto!("rss3");
}

#[derive(Debug)]
struct Rss3Service {
    tx: mpsc::UnboundedSender<Message>,
    // tx_shares: mpsc::UnboundedSender<Shares>,
}

#[tonic::async_trait]
impl Rss3 for Rss3Service {
    async fn send_message(&self, req: Request<Message>) -> Result<Response<Ack>, Status> {
        let msg = req.into_inner();
        self.tx.send(msg).unwrap();
        Ok(Response::new(Ack { ok: true }))
    }

    // async fn send_shares(&self, req: Request<Shares>) -> Result<Response<Ack>, Status> {
    //     let shares = req.into_inner();
    //     self.tx_shares.send(Shares {
    //         share_1half: shares.share_1half,
    //         share_2half: shares.share_2half,
    //     }).unwrap();
    //     Ok(Response::new(Ack { ok: true }))
    // }
}


// Start a gRPC server for a party
pub async fn start_server(port: u16, tx: mpsc::UnboundedSender<Message>) {
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    let svc = Rss3Service { tx };
    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(Rss3Server::new(svc))
        .serve(addr)
        .await
        .unwrap();
}

// Send a message to another party
// pub async fn send_message(to: &str, from_id: i32, payload: &str) {
//     let mut client = Rss3Client::connect(format!("http://{}", to))
//         .await
//         .expect("connect failed");
//     let req = Request::new(Message {
//         from: from_id,
//         payload: payload.to_string()+from_id.to_string().as_str(),
//     });
//     client.send_message(req).await.unwrap();
// }

pub async fn send_message(to: &str, from_ip: String, from_partyid:i32, protocol_number:i32,step_n:i32 ,shares_tuple:(u64,u64)) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Rss3Client::connect(format!("http://{}", to)).await?;
    let req = Request::new(Message {
        from_ip: from_ip,
        from_party_id: from_partyid,
        protocol: protocol_number,
        step: step_n,
        share_1half: shares_tuple.0,
        share_2half: shares_tuple.1,
    });
    client.send_message(req).await?;
    Ok(())
}


// pub async fn send_message_(to: &str, from_id: i32, shares_tuple:(u64,u64)) {
//     loop {
//         match Rss3Client::connect(format!("http://{}", to)).await {
//             Ok(mut client) => {
//                 let req = Request::new(Message {
//                     from: from_id,
//                     protocol: 0,
//                     step: 0,
//                     share_1half: shares_tuple.0,
//                     share_2half: shares_tuple.1,
//                 });
//                 match client.send_message(req).await {
//                     Ok(_) => break, // Success, exit loop
//                     Err(e) => {
//                         eprintln!("Failed to send message: {}. Retrying...", e);
//                         tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//                     }
//                 }
//             }
//             Err(e) => {
//                 eprintln!("Failed to connect to {}: {}. Retrying...", to, e);
//                 tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//             }
//         }
//     }
// }


// pub async fn send_shares(ports: Vec<u16>, party_id:u16, to: &str, from_id: i32, protocol_number:i32, step_n:i32,shares: Vec<u32>) {
//     let my_port = ports[party_id as usize];
//     for port in &ports {
//         println!("port is: {}", port);
//         if *port != my_port {
//             let addr = format!("127.0.0.1:{}", port);
//             // Try to send "ready" message, ignore errors
//         if let Err(e) = send_message(&addr, party_id as i32, protocol_number,step_n,"89757").await {
//             eprintln!("Party {} failed to send ready to {}: {}", party_id, addr, e);
//             // Do not panic, just log and continue
//         } else {
//             println!("Party {} sent ready to {}", party_id, addr);
//         }
//         }
//     }
    
// }


// pub async fn send_shares(to: &str, from_id: i32, share: (u64, u64)) -> Result<(), Box<dyn std::error::Error>> {
//     let mut client = Rss3Client::connect(format!("http://{}", to)).await?;
//     let payload = format!("{},{}", share.0, share.1);
//     let req = Request::new(Shares {
//         share_1half: share.0,
//         share_2half: share.1,
//     });
//     client.send_shares(req).await?;
//     Ok(())
// }