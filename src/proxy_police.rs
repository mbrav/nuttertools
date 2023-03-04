use std::io;
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use crate::Error;

use clap::{arg, Args};

/// Proxy tool for spoofing red light activity
#[derive(Args)]
pub struct Options {
    /// Specify host name to listen on
    #[arg(short = 'a', long, value_name = "VAL", default_value = "127.0.0.1")]
    host: Ipv4Addr,
    /// Specify port number to listen on
    #[arg(short = 'b', long, value_name = "NUM", default_value_t = 8000)]
    port: u16,
    /// Specify host name to forward connection to
    #[arg(short = 'x', long, value_name = "VAL", default_value = "127.0.0.1")]
    forward_host: Ipv4Addr,
    /// Specify port number to forward connection to
    #[arg(short = 'y', long, value_name = "NUM", default_value_t = 8001)]
    forward_port: u16,
}

/// # Errors
///
/// Will return `Err` string is empty
pub fn main(opts: &Options) -> Result<(), Error> {
    let listener = TcpListener::bind(format!("{}:{}", &opts.host, &opts.port))
        .expect("Error when opening socket");

    println!("Listening on: {}:{}", &opts.host, &opts.port);

    for stream in listener.incoming() {
        let incoming_stream = stream.expect("Error unwrapping stream");

        println!(
            "Forwarding to : {}:{}",
            &opts.forward_host, opts.forward_port
        );

        let connection =
            TcpStream::connect(format!("{}:{}", &opts.forward_host, &opts.forward_port))
                .map(|stream| thread::spawn(move || handle_connection(incoming_stream, stream)));

        match connection {
            Ok(_) => {
                println!("Successfully established a connection with client");
            }
            Err(err) => {
                println!("Unable to establish a connection with client {}", err);
            }
        }
    }

    Ok(())
}

fn handle_connection(stream_one: TcpStream, stream_two: TcpStream) {
    print!("Stream one data {:?}", &stream_one);
    print!("Stream two data {:?}", &stream_two);

    let arc_one = Arc::new(stream_one);
    let arc_two = Arc::new(stream_two);

    let mut one_tx = arc_one.try_clone().unwrap();
    let mut one_rx = arc_one.try_clone().unwrap();
    let mut two_tx = arc_two.try_clone().unwrap();
    let mut two_rx = arc_two.try_clone().unwrap();

    let connections = vec![
        thread::spawn(move || io::copy(&mut one_tx, &mut two_rx).unwrap()),
        thread::spawn(move || io::copy(&mut two_tx, &mut one_rx).unwrap()),
    ];

    for connection in connections {
        connection.join().unwrap();
    }
}
