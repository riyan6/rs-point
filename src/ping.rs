use pnet::packet::{
    icmp::{
        echo_reply::EchoReplyPacket,
        echo_request::{IcmpCodes, MutableEchoRequestPacket},
        IcmpTypes,
    },
    ip::IpNextHeaderProtocols,
    util, Packet,
};
use pnet_transport::icmp_packet_iter;
use pnet_transport::TransportChannelType::Layer4;
use pnet_transport::{transport_channel, TransportProtocol};
use rand::random;
use std::{
    net::IpAddr,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

const ICMP_SIZE: usize = 64;

pub async fn ping(target_ip: IpAddr) -> anyhow::Result<Option<Duration>> {
    // 确定协议 并且创建数据包通道 tx 为发送通道, rx 为接收通道
    let protocol = Layer4(TransportProtocol::Ipv4(IpNextHeaderProtocols::Icmp));
    let (mut tx, mut rx) = transport_channel(4096, protocol)?;

    // 将 rx 接收到的数据包传化为 iterator
    let mut iter = icmp_packet_iter(&mut rx);

    let mut icmp_header: [u8; ICMP_SIZE] = [0; ICMP_SIZE];
    let icmp_packet = create_icmp_packet(&mut icmp_header);
    let timer = Arc::new(RwLock::new(Instant::now()));
    // 发送 ICMP 数据包
    tx.send_to(icmp_packet, target_ip)?;

    match iter.next() {
        // 匹配 EchoReplyPacket 数据包
        Ok((packet, addr)) => match EchoReplyPacket::new(packet.packet()) {
            Some(_) => {
                if packet.get_icmp_type() == IcmpTypes::EchoReply {
                    let start_time = timer.read().unwrap();
                    let rtt = Instant::now().duration_since(*start_time);
                    return Ok(Some(rtt));
                }
            }
            None => {}
        },
        Err(e) => {
            println!("An error occurred while reading: {}", e);
        }
    }
    Ok(None)
}

fn create_icmp_packet<'a>(icmp_header: &'a mut [u8]) -> MutableEchoRequestPacket<'a> {
    let mut icmp_packet = MutableEchoRequestPacket::new(icmp_header).unwrap();
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    icmp_packet.set_icmp_code(IcmpCodes::NoCode);
    icmp_packet.set_identifier(random::<u16>());
    icmp_packet.set_sequence_number(1);
    let checksum = util::checksum(icmp_packet.packet(), 1);
    icmp_packet.set_checksum(checksum);

    icmp_packet
}

