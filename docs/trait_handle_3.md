### Part 3: Implementing Traits in Rust for Packet Handling with pnet - Layer 4

In this third part of our tutorial series, we'll continue building on our packet handling application by adding support for Layer 4 packets, specifically TCP and UDP. We'll introduce the `Layer4Infos` struct, create a `PacketPorts` trait for extracting Layer 4 information, and implement the `get_layer_4_infos` function to handle TCP and UDP packets.

### Prerequisites:

- Basic knowledge of Rust.
- Familiarity with the `pnet` library for packet manipulation.
- Completion of Part 2 of this tutorial series where we introduced handling Layer 3 packets.

### Step 1: Defining the Layer4Infos Struct

In Part 3, we introduce the `Layer4Infos` struct, which will hold information specific to Layer 4 network packets, such as source and destination ports.

```rust
#[derive(Debug, Default)]
pub struct Layer4Infos {
    pub port_source: Option<String>,
    pub port_destination: Option<String>,
}
```

The `Layer4Infos` struct is essential for storing Layer 4-specific information.

Let's discuss the derive(Default) aspect for handling the Option that may return None with Some.

In the Layer4Infos struct, we added Default as a derive attribute. Here's how it looks:
When you derive Default for a struct, it automatically provides a default implementation for all its fields. For Option fields like port_source and port_destination, this means they will be initialized with None by default.

This is a useful feature because it ensures that if, for some reason, the extraction of port information (in the ports method) fails or isn't applicable for a given packet (e.g., a non-TCP/UDP packet), these fields will be set to None, and you won't have to explicitly handle the error case.


### Step 2: Creating the `PacketPorts` Trait

In this step, we define the `PacketPorts` trait, which will be implemented by different packet types (e.g., TCP and UDP) to extract Layer 4 information.

```rust
trait PacketPorts {
    fn ports(&self) -> Layer4Infos;
}
```

The `PacketPorts` trait defines a method named `ports` that returns a `Layer4Infos` object containing port information. Different packet types will implement this trait to extract Layer 4 details.

### Step 3: Implementing `PacketPorts` for TCP and UDP

We've implemented the `PacketPorts` trait for TCP and UDP packets. Here's an example of the implementation for `TcpPacket`:

```rust
impl PacketPorts for TcpPacket<'_> {
    fn ports(&self) -> Layer4Infos {
        Layer4Infos {
            port_source: Some(self.get_source().to_string()),
            port_destination: Some(self.get_destination().to_string()),
        }
    }
}
```

Similarly, we implement it for `UdpPacket`. These implementations extract port information from TCP and UDP packets.

By using Some, we are explicitly wrapping the extracted port values in an Option. This is important because it indicates that valid port values are present, and it allows you to distinguish between the case where ports are successfully extracted (with Some) and the case where they couldn't be extracted (with None).

### Step 4: Implementing `get_layer_4_infos` Function

We create a function named `get_layer_4_infos` that takes the IP protocol type (`IpNextHeaderProtocol`) and packet data as input. Depending on the protocol type (TCP, UDP, or others), it delegates to the appropriate packet type and extracts Layer 4 information.

```rust
pub fn get_layer_4_infos(proto: IpNextHeaderProtocol, data: &[u8]) -> Layer4Infos {
    match proto {
        IpNextHeaderProtocols::Tcp => {
            if let Some(tcp_packet) = TcpPacket::new(data) {
                tcp_packet.ports()
            } else {
                Default::default()
            }
        }
        IpNextHeaderProtocols::Udp => {
            if let Some(udp_packet) = UdpPacket::new(data) {
                udp_packet.ports()
            } else {
                Default::default()
            }
        }
        _ => {
            // General case for all other protocols
            println!(
                "Unknown or unsupported protocol: {:?}",
                proto.to_string()
            );
            Default::default()
        }
    }
}
```

This function is the entry point for extracting Layer 4 information from packet data based on the IP protocol.

Since Layer4Infos has been derived with Default, it means that when you call Default::default() on it, you get an instance where the port_source and port_destination fields are initialized with None by default. This is particularly useful when the TcpPacket::new(data) fails, indicating that the packet doesn't contain valid TCP information. By returning a Layer4Infos instance with None values for the port fields, it clearly indicates that no port information is available for this packet.

In summary, Default::default() is used here to provide a meaningful and consistent default value for Layer4Infos when the extraction of TCP packet information fails, ensuring that the returned value accurately represents the absence of valid port data.

### Conclusion

In this third part of our tutorial series, we've extended our packet handling application to include Layer 4 network packets, specifically TCP and UDP. We've defined the `Layer4Infos` struct, introduced the `PacketPorts` trait for extracting Layer 4 information, and implemented the `get_layer_4_infos` function to handle TCP and UDP packets.

With the completion of this part, your packet handling application is now capable of processing Layer 4 packets, making it more comprehensive and versatile. In the next part of the series, we can explore more advanced topics or enhancements to further improve your packet handling capabilities.