use nanomsg_rust::{NanomsgRequest, NanomsgReply};
use std::io;


#[tokio::main]
async fn main() {
    tokio::spawn( async {
        if let Err(err) = run_reply_task(4536).await {
            eprintln!("Listener Err: {}", err);
        }
    });
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    if let Err(err) = run_request_task("localhost:4536").await {
        eprintln!("Client Err: {}", err);
    }
}


async fn run_reply_task(port: u16) -> io::Result<()> {
    let mut reply_socket = NanomsgReply::new(port, handle_request, false);
    reply_socket.bind().await?;
    println!("Serving");
    reply_socket.serve().await
}

async fn handle_request(req: Vec<u8>) -> Vec<u8> {
    println!("Reply incoming request: {}", String::from_utf8_lossy(&req));
    return b"pong".to_vec()
}

async fn run_request_task(address: &str) -> io::Result<()> {
    let socket = NanomsgRequest::new(address.to_string(), false);
    println!("Requesting");
    let reply_packet = socket.request(b"ping".to_vec()).await?;

    println!("Request socket incoming reply: {}", String::from_utf8_lossy(&reply_packet));
    
    Ok(())
}

