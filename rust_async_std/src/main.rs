extern crate async_std;

use async_std::io;
use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task;

fn main() -> io::Result<()> {
    task::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8081").await?;
        println!("Listening on {}", listener.local_addr()?);

        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let mut stream = stream?;
            task::spawn(async move {
                let mut length: usize = 0;
                let mut buf = [0; 1024];

                loop {
                    // read
                    let r = stream.read(&mut buf[length..]).await;
                    match r {
                        Ok(n) => {
                            length += n;
                        }
                        Err(_) => {
                            return
                        }
                    }
                    if buf[0..length].ends_with(b"\r\n\r\n") {
                        // write
                        let res = stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 11\r\n\r\nhello world").await;
                        match res {
                            Ok(_)=> {
                                // 发送成功
                                length = 0
                            }
                            Err(_) => {
                                println!("write error")
                            }
                        }
                    }
                }
            });
        }
        Ok(())
    })
}
