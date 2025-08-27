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
}

#[tonic::async_trait]
impl Rss3 for Rss3Service {
    async fn send_message(&self, req: Request<Message>) -> Result<Response<Ack>, Status> {
        let msg = req.into_inner();
        self.tx.send(msg).unwrap();
        Ok(Response::new(Ack { ok: true }))
    }
}

// Start a gRPC server for a party
async fn start_server(port: u16, tx: mpsc::UnboundedSender<Message>) {
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
async fn send_message(to: &str, from_id: i32, payload: &str) {
    let mut client = Rss3Client::connect(format!("http://{}", to))
        .await
        .expect("connect failed");
    let req = Request::new(Message {
        from: from_id,
        payload: payload.to_string()+from_id.to_string().as_str(),
    });
    client.send_message(req).await.unwrap();
}

#[tokio::main]
async fn main() {
    // Example setup: 3 parties
    let party_id: usize = std::env::args().nth(1).unwrap().parse().unwrap();
    let ports = vec![5000, 5001, 5002];
    let my_port = ports[party_id];

    let (tx, mut rx) = mpsc::unbounded_channel();

    // Start gRPC server
    tokio::spawn(start_server(my_port, tx));

    // Give the server some time to start
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // Send messages to the other parties
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
