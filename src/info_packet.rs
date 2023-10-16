use pnet::packet::{ethernet::{EthernetPacket, EtherTypes},
                            ipv6::Ipv6Packet, 
                            Packet,
                };

pub fn process_packet_by_type(interface_name: &str, ethernet_packet: &EthernetPacket<'_>) {
    print_packet_layer_2(interface_name, ethernet_packet);
    match ethernet_packet.get_ethertype() {
        EtherTypes::Ipv6 => {
            if let Some(ipv6_packet) = Ipv6Packet::new(ethernet_packet.payload()) {
                println!(
                    "Layer 3: IPv6 packet: source {} destination {} => {} {}",
                    ipv6_packet.get_source(),
                    ipv6_packet.get_destination(),
                    ipv6_packet.get_next_header(),
                    ipv6_packet.get_payload_length()
                );
            }
        }
        EtherTypes::Ipv4 => {println!("ipv4")},
        EtherTypes::Arp => {println!("Arp")},
        _ => {
            // General case for all other EtherTypes
            println!(
                "Unknown or unsupported packet type: {:?}",
                ethernet_packet.get_ethertype()
            );
        }
    }
}

fn print_packet_layer_2(interface_name: &str, ethernet_packet: &EthernetPacket<'_>) {
    println!(
        "Layer 2: New packet on {}: {} => {}: {}",
        interface_name,
        ethernet_packet.get_source(),
        ethernet_packet.get_destination(),
        ethernet_packet.get_ethertype()
    );
}
