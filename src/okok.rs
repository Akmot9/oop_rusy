use pnet::packet::{ethernet::{EthernetPacket, EtherTypes},
                    ipv6::Ipv6Packet, 
                    Packet, ipv4::Ipv4Packet, arp::ArpPacket,
                };

pub fn process_packet_by_type(interface_name: &str, ethernet_packet: &EthernetPacket<'_>) {
    print_packet_layer_2(interface_name, ethernet_packet);
    match ethernet_packet.get_ethertype() {
        EtherTypes::Ipv6 => {get_layer_3_infos(ethernet_packet.payload())},
        EtherTypes::Ipv4 => {
            if let Some(ipv4_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
                println!(
                    "Layer 3: IPv4 packet: source {} destination {} => {} {}",
                    ipv4_packet.get_source(),
                    ipv4_packet.get_destination(),
                    ipv4_packet.get_next_level_protocol(),
                    ipv4_packet.get_total_length(),
                );
            }
        },
        EtherTypes::Arp => {
            if let Some(arp_packet) = ArpPacket::new(ethernet_packet.payload()) {
                println!(
                    "Layer 2: arp packet: source {} destination {} => {:?} {} {} {:?} {}",
                    arp_packet.get_sender_hw_addr(),
                    arp_packet.get_target_hw_addr(),
                    arp_packet.get_operation(),
                    arp_packet.get_target_proto_addr(),
                    arp_packet.get_sender_proto_addr(),
                    arp_packet.get_hardware_type(),
                    arp_packet.get_proto_addr_len()
                );
            }
        },
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

fn get_layer_3_infos(payload: &[u8]) {
    if let Some(ipv6_packet) = Ipv6Packet::new(payload) {
        // Get the payload_length field
        let payload_length = ipv6_packet.get_payload_length();
        println!("Payload Length: {}", payload_length);

        // Get the next_header field
        let next_header = ipv6_packet.get_next_header();
        println!("Next Header: {:?}", next_header);

        // Get the source field
        let source = ipv6_packet.get_source();
        println!("Source: {}", source);

        // Get the destination field
        let destination = ipv6_packet.get_destination();
        println!("Destination: {}", destination);
    }
}