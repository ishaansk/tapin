use std::net::{TcpListener,TcpStream};
use std::thread;
use std::time::Duration;
use std::io::{Read,Write};

fn handle_client(mut stream: TcpStream)->Result<(), Box<dyn std::error::Error>>{
    let clientaddr = stream.peer_addr().unwrap();
    println!("started address = {}",clientaddr);
    let mut buffer = [0u8;1024];
        loop{
        let n = stream.read(&mut buffer)?;
        if n==0{
            break;
        }
        println!("the bytes = {:?}",&buffer[..n]);
        stream.write_all(&mut buffer[..n])?;
    }
    //thread::sleep(Duration::from_secs(10));
    println!("ended address = {}", clientaddr);
    Ok(())
}

fn main() -> Result<(),Box<dyn std::error::Error>>{
    let listener = TcpListener::bind("127.0.0.1:8080").expect("failed to build listener");
    println!("Starting to listen on {:?}",listener);
    for stream in listener.incoming(){
        let stream = stream?;
        std::thread::spawn(move || {
        handle_client(stream);
        });
    }
    Ok(())
}
