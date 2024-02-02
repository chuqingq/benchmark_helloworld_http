extern crate async_std;

use async_std::io;
use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task;

const RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Length: 11\r\n\r\nhello world";

fn main() -> io::Result<()> {
    task::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8081").await?;
        println!("Listening on {}", listener.local_addr()?);

        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let mut stream = stream?;
            task::spawn(async move {
                let mut buflen: usize = 0;
                let mut buf = vec![0; 256];
                // let mut buf = [0; 256];

                loop {
                    // read
                    let r = stream.read(&mut buf[buflen..]).await;
                    match r {
                        Ok(n) if n != 0 => buflen += n,
                        Ok(_) => return,
                        Err(_) => {
                            // println!("read error: {e}");
                            return;
                        }
                    }
                    if buf[0..buflen].ends_with(b"\r\n\r\n") {
                        // write
                        let res = stream.write_all(RESPONSE).await;
                        match res {
                            Ok(_) => buflen = 0,
                            Err(e) => {
                                println!("write error: {e}");
                                return;
                            }
                        }
                    }
                }
            });
        }
        println!("server stopped");
        Ok(())
    })
}
