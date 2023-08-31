use log;
use std::sync::Arc;
use std::{collections::HashMap, str};

use tokio::{io, net::TcpListener, sync::Mutex};

pub mod command;
pub mod resp;
// pub mod command;

#[tokio::main]
async fn main() {
    log::info!("Logs from your program will appear here!");

    const HOST: &str = "127.0.0.1";
    const PORT: &str = "6379";
    let redis_storage: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind(format!("{}:{}", HOST, PORT))
        .await
        .unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let redis_storage = redis_storage.clone();

        let _handle = tokio::spawn(async move {
            let mut input_buffer: [u8; 512] = [0; 512];
            
            'read_process: loop {
                match socket.readable().await {
                    Ok(_) => match socket.try_read(&mut input_buffer) {
                        Ok(0) => {
                            log::error!(
                                "Received void message!\n
                                 It is impossible!"
                            );
                            break 'read_process;
                        }
                        Ok(size) => {
                            let mut redis_storage = redis_storage.lock().await;

                            let request_decode_result = str::from_utf8(&input_buffer[..size]);
                            match request_decode_result {
                                Ok(request) => {
                                    log::debug!("read {} bytes", size);
                                    log::debug!("read `{:?}` message", request);

                                    let response: resp::Message =
                                        command::process_request(request, &mut redis_storage);

                                    match socket.writable().await {
                                        Ok(_) => {
                                            match socket
                                                .try_write(String::from(&response).as_bytes())
                                            {
                                                Ok(_) => {}
                                                Err(_) => {}
                                            }
                                        }
                                        Err(_) => {
                                            println!("match socket.writable().await {{");
                                        }
                                    }
                                }
                                Err(_) => {
                                    println!("Ok(request) => {{");
                                }
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
