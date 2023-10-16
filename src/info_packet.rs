use std::fmt;

use pnet::packet::ethernet::EthernetPacket;
use pnet::util::MacAddr;

use crate::info_packet::layer_3_infos::{get_layer_3_infos, Layer3Infos};
mod layer_3_infos;

pub struct PacketInfos {
    mac_address_source: MacAddr,
    mac_address_destination: MacAddr,
    interface: String,
    layer_3_infos: Layer3Infos
}

impl PacketInfos {
    pub fn new(interface_name: &String, ethernet_packet: &EthernetPacket<'_>) -> PacketInfos{
        
        PacketInfos {
            mac_address_source: ethernet_packet.get_source(),
            mac_address_destination: ethernet_packet.get_destination(),
            interface: interface_name.to_string(),
            layer_3_infos: get_layer_3_infos(ethernet_packet)
        }
    }
}

impl fmt::Display for PacketInfos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Implement the formatting for PacketInfos here
        write!(f, "MAC Source: {}\n", self.mac_address_source)?;
        write!(f, "MAC Destination: {}\n", self.mac_address_destination)?;
        write!(f, "Interface: {}\n", self.interface)?;
        write!(f, "ip_source: {}\n", self.layer_3_infos.ip_source)?;
        write!(f, "ip_source: {}\n", self.layer_3_infos.ip_destination)?;

        // Format other fields as needed

        Ok(())
    }
}