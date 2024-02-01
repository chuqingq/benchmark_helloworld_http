// You can run this example from the root of the mio repo:
// cargo run --example tcp_server --features="os-poll net"
use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Registry, Token};
use std::collections::HashMap;
use std::io::{self, Read, Write};
// use std::str::from_utf8;

// Setup some tokens to allow us to identify which event is for which socket.
const SERVER: Token = Token(0);

// Some data we'll send over the connection.
const DATA: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Length: 11\r\n\r\nhello world";

struct Conn {
    stream: TcpStream,
    data: Vec<u8>,
    len: usize,
}

#[cfg(not(target_os = "wasi"))]
fn main() -> io::Result<()> {
    // env_logger::init();

    // Create a poll instance.
    let mut poll = Poll::new()?;
    // Create storage for events.
    let mut events = Events::with_capacity(128);

    // Setup the TCP server socket.
    let addr = "127.0.0.1:8081".parse().unwrap();
    let mut server = TcpListener::bind(addr)?;

    // Register the server with poll we can receive events for it.
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    // Map of `Token` -> `TcpStream`.
    let mut connections = HashMap::new();
    // Unique token for each incoming connection.
    let mut unique_token = Token(SERVER.0 + 1);

    // println!("You can connect to the server using `nc`:");
    // println!(" $ nc 127.0.0.1 9000");
    // println!("You'll see our welcome message and anything you type will be printed here.");

    loop {
        if let Err(err) = poll.poll(&mut events, None) {
            if interrupted(&err) {
                continue;
            }
            return Err(err);
        }

        for event in events.iter() {
            match event.token() {
                SERVER => loop {
                    // Received an event for the TCP server socket, which
                    // indicates we can accept an connection.
                    let (mut connection, _address) = match server.accept() {
                        Ok((connection, address)) => (connection, address),
                        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                            // If we get a `WouldBlock` error we know our
                            // listener has no more incoming connections queued,
                            // so we can return to polling and wait for some
                            // more.
                            break;
                        }
                        Err(e) => {
                            // If it was any other kind of error, something went
                            // wrong and we terminate with an error.
                            return Err(e);
                        }
                    };

                    // println!("Accepted connection from: {}", address);

                    let token = next(&mut unique_token);
                    poll.registry().register(
                        &mut connection,
                        token,
                        Interest::READABLE, // .add(Interest::WRITABLE)
                    )?;

                    let conn = Conn {
                        stream: connection,
                        data: vec![0; 1024],
                        len: 0,
                    };
                    connections.insert(token, conn);
                },
                token => {
                    // Maybe received an event for a TCP connection.
                    let done = if let Some(connection) = connections.get_mut(&token) {
                        handle_connection_event(poll.registry(), connection, event)?
                    } else {
                        // Sporadic events happen, we can safely ignore them.
                        false
                    };
                    if done {
                        if let Some(mut connection) = connections.remove(&token) {
                            poll.registry().deregister(&mut connection.stream)?;
                        }
                    }
                }
            }
        }
    }
}

fn next(current: &mut Token) -> Token {
    let next = current.0;
    current.0 += 1;
    Token(next)
}

/// Returns `true` if the connection is done.
fn handle_connection_event(
    registry: &Registry,
    conn: &mut Conn,
    event: &Event,
) -> io::Result<bool> {
    let connection = &mut conn.stream;
    if event.is_writable() {
        // TODO chuqq
        // We can (maybe) write to the connection.
        match connection.write(DATA) {
            // We want to write the entire `DATA` buffer in a single go. If we
            // write less we'll return a short write error (same as
            // `io::Write::write_all` does).
            Ok(n) if n < DATA.len() => return Err(io::ErrorKind::WriteZero.into()),
            Ok(_) => {
                // println!("write {n} bytes");
                // After we've written something we'll reregister the connection
                // to only respond to readable events.
                registry.reregister(connection, event.token(), Interest::READABLE)?
            }
            // Would block "errors" are the OS's way of saying that the
            // connection is not actually ready to perform this I/O operation.
            Err(ref err) if would_block(err) => {}
            // Got interrupted (how rude!), we'll try again.
            Err(ref err) if interrupted(err) => {
                return handle_connection_event(registry, conn, event)
            }
            // Other errors we'll consider fatal.
            Err(_err) => {
                // println!("write error: {err}");
                return Ok(true)
            }
        }
    }

    if event.is_readable() {
        let mut connection_closed = false;
        // let received_data = &mut conn.data;
        // let bytes_read = &mut conn.len;
        // We can (maybe) read from the connection.
        loop {
            match connection.read(&mut conn.data[conn.len..]) {
                Ok(0) => {
                    // Reading 0 bytes means the other side has closed the
                    // connection or is done writing, then so are we.
                    connection_closed = true;
                    break;
                }
                Ok(n) => {
                    // println!("recv {n} bytes");
                    conn.len += n;
                    if conn.data[conn.len - 4] == b'\r'
                        && conn.data[conn.len - 3] == b'\n'
                        && conn.data[conn.len - 2] == b'\r'
                        && conn.data[conn.len - 1] == b'\n'
                    {
                        registry.reregister(connection, event.token(), Interest::WRITABLE)?
                    }
                    // if bytes_read == received_data.len() {
                    //     received_data.resize(received_data.len() + 1024, 0);
                    // }
                }
                // Would block "errors" are the OS's way of saying that the
                // connection is not actually ready to perform this I/O operation.
                Err(ref err) if would_block(err) => break,
                Err(ref err) if interrupted(err) => continue,
                // Other errors we'll consider fatal.
                Err(_err) => {
                    // println!("read error: {err}");
                    return Ok(true)
                }
            }
        }

        // if bytes_read != 0 {
        //     let received_data = &received_data[..bytes_read];
        //     if let Ok(str_buf) = from_utf8(received_data) {
        //         println!("Received data: {}", str_buf.trim_end());
        //     } else {
        //         println!("Received (none UTF-8) data: {:?}", received_data);
        //     }
        // }

        if connection_closed {
            // println!("Connection closed");
            return Ok(true);
        }
    }

    Ok(false)
}

fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

fn interrupted(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::Interrupted
}

#[cfg(target_os = "wasi")]
fn main() {
    panic!("can't bind to an address with wasi")
}
