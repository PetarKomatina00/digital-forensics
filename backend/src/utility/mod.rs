use axum::{routing::get, Router};

use crate::{constants, repository::load_pcap_packets};

pub fn get_header_len(data: &u8) -> u8{
    let x = (data & 0xF0) >> 4;
    x
}
pub fn convert_8_to_16(data: &[u8]) -> u16{
    (data[0] as u16) << 8 | data[1] as u16
}
pub fn convert_8_to_32(data: &[u8]) -> u32{
    ((data[0] as u32) << 24) | ((data[1] as u32) << 16)
    | ((data[2] as u32) << 8) | data[3] as u32 
}
pub fn convert_8_to_40(data: &[u8]) -> u64{
    let mut array = [0u8; 8];
    let len = data.len().min(8);
    array[..len].copy_from_slice(&data[..len]);
    u64::from_be_bytes(array)
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