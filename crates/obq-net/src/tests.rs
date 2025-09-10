use tokio::sync::mpsc;
use crate::network_lib::{start_server, send_message};



#[tokio::test]
async fn test_server() {
    let party_id: usize = std::env::args().nth(1).unwrap().parse().unwrap();
    let ports = vec![5000, 5001, 5002];
    let my_port = ports[party_id];

    let (tx, mut rx) = mpsc::unbounded_channel();

    // Start gRPC server
    tokio::spawn(start_server(my_port, tx));
    println!("Party {} is ready! Server started on port {}", party_id ,my_port);
    for (i, port) in ports.iter().enumerate() {
        if i != party_id {
            let addr = format!("127.0.0.1:{}", port); // replace 127.0.0.1 with remote IP for different machines
            println!("Party {} sending to {}", party_id, addr);
            send_message(&addr, party_id as i32, "Hello from party").await;
        }
    }

    // Receive messages
    while let Some(msg) = rx.recv().await {
        println!("Party {} received from {}: {}", party_id, msg.from, msg.payload);
    }

} 