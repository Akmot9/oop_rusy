use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::EthernetPacket;
use std::thread;

use layer_2_infos::PacketInfos;
mod layer_2_infos;

fn main() {
    let interfaces = datalink::interfaces();
    let mut handles = vec![];

    for interface in interfaces {
        let handle = thread::spawn(move || {
            capture_packets(interface);
        });
        handles.push(handle);
    }
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}

fn capture_packets(interface: datalink::NetworkInterface) {
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type: {}", &interface),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    println!("Start thread reading packet on interface: {}", &interface);
    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                    println!("---");
                    let packet_info = PacketInfos::new(&interface.name, &ethernet_packet);
                    println!("{}", packet_info);
                    
                }
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use pnet::datalink::dummy::{dummy_interface, interfaces};

    #[test]
    fn test_dummy_interface_creation() {
        // Create a dummy interface
        let dummy = dummy_interface(0);
        println!("{}",&dummy);

        // Obtain a list of dummy interfaces
        let dummy_interfaces = interfaces();
        println!("{:?}",&dummy_interfaces);

        // Assert that the created dummy interface is in the list
        assert!(dummy_interfaces.contains(&dummy), "Dummy interface not found in the list");

        // Assert the presence of MAC address (it's an Option)
        assert!(dummy.mac.is_some(), "MAC address is not present");

        // You can also assert other properties of the dummy interface if needed
        assert_eq!(dummy.name, "eth0", "Unexpected interface name");
        assert_eq!(dummy.index, 0, "Unexpected interface index");
    }
}
