use clap::{Arg, Command};
use std::io::{Error, ErrorKind};
use std::net::UdpSocket;

fn send_packet(mac: &str, host: &str) -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;

    let hex_mac = match hex::decode(mac.replace(":", "")) {
        Ok(mac) => mac,
        Err(_reason) => return Err(Error::new(ErrorKind::InvalidInput, "Invalid MAC_ID!")), //: {:?}", reason),
    };

    let mut packet = vec![0u8; 102];

    for i in 0..6 {
        packet[i] = 0xFF;
    }

    for i in 0..16 {
        for j in 0..6 {
            packet[6 + (i * 6) + j] = hex_mac[j];
        }
    }

    //println!("{}", host);
    //println!("{:?}", packet);

    socket.send_to(&packet, (host, 9))?;

    Ok(())
}

fn main() {
    let matches = Command::new("Wake on Lan")
        .version("1.0")
        .author("Jeffry Utter <jeff@jeffutter.com>")
        .about("Sends a Magic Packet to wake a remote machine")
        .arg(
            Arg::new("host")
                .short('h')
                .long("host")
                .value_name("HOST")
                .help("The host to send the packet to")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("mac")
                .short('m')
                .long("mac")
                .value_name("MAC_ID")
                .help("The MAC ID of the machine to wake")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let mac = matches.value_of("mac").unwrap();
    let host = matches.value_of("host").unwrap();

    match send_packet(mac, host) {
        Ok(()) => {
            println!("Magic Packet Sent!");
            std::process::exit(0);
        }
        Err(reason) => {
            eprintln!("Failed to send Magic Packet: {}", reason.to_string());
            std::process::exit(1);
        }
    };
}
