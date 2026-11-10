use std::net::{TcpListener,TcpStream};


fn handle_client(stream: TcpStream){
    let clientaddr = stream.peer_addr().unwrap();
    println!("client address = {}",clientaddr);

}

fn main() -> Result<(),Box<dyn std::error::Error>>{
    let listener = TcpListener::bind("127.0.0.1:8080").expect("failed to build listener");
    println!("Starting to listen on {:?}",listener);
    for stream in listener.incoming(){
        handle_client(stream?);
    }
    Ok(())
}
