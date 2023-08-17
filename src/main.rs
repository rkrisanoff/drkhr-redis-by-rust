use std::str;

use tokio::{io, net::TcpListener};

pub mod resp;
// use resp::{form_response, DataType};
use log;

#[tokio::main]
async fn main() {
    log::info!("Logs from your program will appear here!");

    const HOST: &str = "127.0.0.1";
    const PORT: &str = "6379";

    let listener = TcpListener::bind(format!("{}:{}", HOST, PORT))
        .await
        .unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let _handle = tokio::spawn(async move {
            let mut input_buffer: [u8; 512] = [0; 512];

            'read_process: loop {
                match socket.readable().await {
                    Ok(_) => match socket.try_read(&mut input_buffer) {
                        Ok(0) => {
                            log::error!(
                                "Received void message!
                            It is impossible!"
                            );
                            break 'read_process;
                        }
                        Ok(size) => {
                            let request_decode_result = str::from_utf8(&input_buffer[..size]);
                            match request_decode_result {
                                Ok(request) => {
                                    log::debug!("read {} bytes", size);
                                    log::debug!("read `{:?}` message", request);
                                    fn check_message_equals(
                                        origin: &[u8],
                                        size: usize,
                                        bench: &str,
                                    ) -> bool {
                                        (origin[..size]).iter().eq(bench.as_bytes().iter())
                                    }
                                    if check_message_equals(&input_buffer, size, "exit\n") {
                                        break 'read_process;
                                    }

                                    match socket.writable().await {
                                        Ok(_) => {
                                            match socket.try_write(
                                                "".as_bytes(), // form_response(DataType::SimpleString, "PONG").as_bytes(),
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
