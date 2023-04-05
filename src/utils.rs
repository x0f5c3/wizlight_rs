use color_eyre::eyre::WrapErr;
use color_eyre::Result;
use pnet::datalink::interfaces;

use rayon::prelude::*;
use socket2::{Domain, Protocol, Socket, Type};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use time::format_description;

pub(crate) fn get_timestamp() -> Result<String> {
    let fmt = format_description::parse("[day]_[month]_[year]_[hour]_[minute]")?;
    time::OffsetDateTime::now_local()?
        .format(&fmt)
        .wrap_err("Failed to format timestamp")
}

pub(crate) fn hex_to_percent(hex_val: f64) -> f64 {
    ((hex_val / 255.0) * 100.0).round()
}

pub(crate) fn percent_to_hex(percent: f64) -> i64 {
    ((percent / 100.0) * 255.0).round() as i64
}

pub fn create_udp_broadcast(listen_port: u16) -> Result<UdpSocket> {
    create_udp(listen_port, true, true)
}

pub fn create_udp_socket(listen_port: u16) -> Result<UdpSocket> {
    create_udp(listen_port, false, false)
}

// pub async fn create_tokio_udp(listen_port: u16, reuseaddr: bool, broadcast: bool) -> Result<UdpSocket> {
//     let sock = UdpSocket::
// }

pub fn get_local_adddrs() -> Vec<String> {
    interfaces()
        .into_par_iter()
        .map(|x| {
            x.ips
                .into_par_iter()
                .filter_map(|x| format!("{}", x).split('/').next().map(|x| x.to_string()))
        })
        .flatten()
        .collect()
}

pub fn create_udp(listen_port: u16, reuseaddr: bool, broadcast: bool) -> Result<UdpSocket> {
    let sock = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))
        .wrap_err("Can't create the socket")?;
    if reuseaddr {
        sock.set_reuse_address(true)?;
    }
    if broadcast {
        sock.set_broadcast(true)?;
    }
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), listen_port);
    let addr = addr.into();
    sock.bind(&addr)?;
    Ok(sock.into())
}
