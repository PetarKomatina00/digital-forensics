use axum::{routing::get, Router};

use crate::{constants, repository::load_pcap_packets};

pub fn convert_8_to_16(data: &[u8]) -> u16{
    (data[0] as u16) << 8 | data[1] as u16
}
pub fn convert_8_to_32(data: &[u8]) -> u32{
    ((data[0] as u32) << 24) | ((data[1] as u32) << 16)
    | ((data[2] as u32) << 8) | data[3] as u32 
}
pub fn mac_to_string(packet: &[u8]) -> String{
    packet.iter()
    .map(|byte| format!("{:02x}", byte))
    .collect::<Vec<String>>()
    .join(":")
}
pub fn ip_addr_to_string(packet: &[u8]) -> String{
    packet.iter()
    .map(|byte| format!("{}", byte))
    .collect::<Vec<String>>()
    .join(".")
}

// pub fn Router() -> Router{
//     Router::new()
//         .route("/", get(load_pcap_packets(constants::PCAP_FILE)))
// }