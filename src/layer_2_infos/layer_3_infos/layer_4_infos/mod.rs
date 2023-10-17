use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::ip::{IpNextHeaderProtocols, IpNextHeaderProtocol};

pub struct Layer4Infos {
    pub port_source: String,
    pub port_destination: String,
}
enum Layer4Paket<'a> {
    Tcp(TcpPacket<'a>),
    Udp(UdpPacket<'a>)
}
struct TcpHandler;
struct UdpHandler;

trait HandlePacketTypes {
    fn get_layer_4(data: &[u8]) -> Layer4Infos;
}

impl HandlePacketTypes for TcpHandler {
    fn get_layer_4(data: &[u8]) -> Layer4Infos {
        if let Some(l_4) = TcpPacket::new(data) {
            get_port_from_tcp_packet(l_4)
        } else {
            get_n_a_4_info()
        }
    }
}

impl HandlePacketTypes for UdpHandler {
    fn get_layer_4(data: &[u8]) -> Layer4Infos {
        if let Some(l_4) = UdpPacket::new(data) {
            get_port_from_udp_packet(l_4)
        } else {
            get_n_a_4_info()
        }
    }
}

fn get_port_from_udp_packet(l_4: UdpPacket) -> Layer4Infos{
    Layer4Infos {
        port_source: l_4.get_source().to_string(),
        port_destination: l_4.get_destination().to_string(),
    }
}

fn get_port_from_tcp_packet(l_4: TcpPacket) -> Layer4Infos{
    Layer4Infos {
        port_source: l_4.get_source().to_string(),
        port_destination: l_4.get_destination().to_string(),
    }
}
fn get_n_a_4_info() -> Layer4Infos {
    Layer4Infos {
        port_source: String::from("N/A"),
        port_destination: String::from("N/A"),
    }
}

pub fn get_layer_4_infos(proto: IpNextHeaderProtocol, data: &[u8]) -> Layer4Infos{
    match proto {
        IpNextHeaderProtocols::Tcp => TcpHandler::get_layer_4(data),
        IpNextHeaderProtocols::Udp => UdpHandler::get_layer_4(data),
        _ => {
            // General case for all other EtherTypes
            println!(
                "Unknown or unsupported packet type: {:?}",
                proto.to_string()
            );
            get_n_a_4_info()
        }
    }
}