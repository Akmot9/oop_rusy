use polars::prelude::*;

pub struct DataFrameHandler {
    // Define fields or methods for DataFrame handling
}

impl DataFrameHandler {
    pub fn new() -> Self {
        // Initialize DataFrameHandler
        Self {
            // Initialize fields or perform setup
        }
    }

    pub fn add_to_data_frame(&mut self, packet_info: &PacketInfos, df: &mut DataFrame) {
        // Add packet_info to df
        // Example:
        // df.add_series("mac_address_source", &packet_info.mac_address_source.to_string());
        // df.add_series("mac_address_destination", &packet_info.mac_address_destination.to_string());
        // ...
    }

    // Define other methods for DataFrame operations as needed
}