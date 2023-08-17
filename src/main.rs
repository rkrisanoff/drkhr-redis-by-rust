use std::str;

use tokio::{
    io,
    net::TcpListener,
};

pub mod resp;
use resp::{form_response, DataType};

#[tokio::main]
async fn main() {
    println!("Logs from your program will appear here!");

    const HOST: &str = "127.0.0.1";
    const PORT: &str = "6379";

    let listener = TcpListener::bind(String::new() + HOST + ":" + PORT)
        .await
        .unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let _handle = tokio::spawn(async move {
            let mut input_buffer: [u8; 512] = [0; 512];

            'read_process: loop {
                match socket.readable().await {
                    Ok(_) => match socket.try_read(&mut input_buffer) {
                        Ok(0) => break 'read_process,
                        Ok(size) => {
                            let request_decode_result = str::from_utf8(&input_buffer[..size]);
                            match request_decode_result {
                                Ok(request) => {
                                    println!("read {} bytes", size);
                                    println!("read `{:?}` message", request);
                                    if (&input_buffer[..size])
                                        .iter()
                                        .eq("exit\n".as_bytes().iter())
                                    {
                                        break 'read_process;
                                    }
                                    if (&input_buffer[..size])
                                        .iter()
                                        .eq("exit_complete\n".as_bytes().iter())
                                    {
                                        return ();
                                    }

                                    match socket.writable().await {
                                        Ok(_) => {
                                            match socket.try_write(
                                                form_response(DataType::String, "PONG").as_bytes(),
                                            ) {
                                                Ok(_) => {}
                                                Err(_) => {}
                                            }
                                        }
                                        Err(_) => {}
                                    }
                                }
                                Err(_) => {}
                            }
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            continue;
                        }
                        Err(e) => {
                            println!("{}", e);
                        }
                    },
                    Err(_) => {}
                };
            }
        });
    }
}
