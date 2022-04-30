use clap::{Arg, Command};
use std::net::UdpSocket;
use anyhow::{Context, Result};

fn send_packet(mac: &str, host: &str) -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;

    let hex_mac = hex::decode(mac.replace(":", "")).context("Invalid MAC_ID!")?;

    let mut packet = vec![0u8; 102];

    for i in 0..6 {
        packet[i] = 0xFF;
    }

    for i in 0..16 {
        for j in 0..6 {
            packet[6 + (i * 6) + j] = hex_mac[j];
        }
    }

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
