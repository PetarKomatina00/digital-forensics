use axum::{routing::get, Router};
use pcap::{Capture, Packet};
use crate::endpoints::get_filter_packets;
use crate::models::{DataLinkFrame, Datagram, ErrorResponse, IpPacketV4, PacketFilter, PacketInfo};
use crate::{constants, endpoints, utility};

pub fn get_data_link_frame(packet: &Packet<'_>, packet_info: &mut PacketInfo) {
    
    let data_link_frame = DataLinkFrame {
        src_mac : utility::mac_to_string(&packet[0..6]),
        dst_mac : utility::mac_to_string(&packet[6..12]),
        ethertype : utility::convert_8_to_16(&packet[12..14])
    };
    packet_info.data_link = Some(data_link_frame);
    
}
pub fn get_ipv4_internet_packet(packet: &Packet<'_>, packet_info: &mut PacketInfo){
    let ip_packet = IpPacketV4 {
        src: utility::ip_addr_to_string(&packet[26..30]),
        dst: utility::ip_addr_to_string(&packet[30..34]),
        protocol: packet[23],
        ttl: packet[22]
    };
    packet_info.ip = Some(ip_packet);
    // let version_header = packet[14];
    // let version = version_header & 0xF0;
    // let version = if version == 64 { "4" } else {"6"};
    // let header = version_header & 0x0F;
    // println!("Version: {}", version);
    // println!("Header: {}", header);
    // println!("Differentiated SF: {}", &packet[15]);
    // let total_length_arr = &packet[16..18];
    // let total_length: u16 = ((total_length_arr[0] as u16) << 8) | total_length_arr[1] as u16;
    // println!("Total Length: {}", total_length);
    // let identification = ((packet[18] as u16) << 8) | packet[19] as u16;
    // println!("Identification: {:?}", identification);
    // let flag_offset_u8 = &packet[20..22];
    // let flag_offset_u16: u16 = ((flag_offset_u8[0] as u16) << 8) | flag_offset_u8[1] as u16;
    // let flag = ((flag_offset_u16 & 0xE000) >> 13) as u8;
    // let offset = flag_offset_u16 & 0x1FFF;
    // println!("Flag: {}", flag);
    // println!("Offset: {}", offset);
    // println!("Time to Live: {:?}", packet[22]);
    // println!("Protocol: {:?}", packet[23]);
    // let header_checksum = (packet[24] as u16) << 8 | packet[25] as u16;
    // println!("Header Checksum: 0x{:X?}", header_checksum);
    // let source_arr = &packet[26..30];
    // let source: String = source_arr[0].to_string() + "." + &source_arr[1].to_string()
    // + "." + &source_arr[2].to_string() + "." + &source_arr[3].to_string();
    // println!("Source: {}", source);
    // let destination_arr  = &packet[30..34];
    // let destination: String = destination_arr[0].to_string() + "." + &destination_arr[1].to_string()
    // + "." + &destination_arr[2].to_string() + "." + &destination_arr[3].to_string();
    // println!("Destination: {}", destination);

}
pub fn get_transport_datagram_tcp(packet: &Packet<'_>, packet_info: &mut PacketInfo){
    if packet_info.ip.as_ref().unwrap().protocol == 6{
        // TCP
        let datagram = Datagram
        { 
            src_port: utility::convert_8_to_16(&packet[34..36]), 
            dst_port: utility::convert_8_to_16(&packet[36..38]), 
            seq: utility::convert_8_to_32(&packet[38..42]), 
            ack: utility::convert_8_to_32(&packet[42..46]), 
            flags: utility::convert_8_to_16(&packet[46..48]) & 0x0FFF
        };

        packet_info.transport = Some(datagram);
    }

    // let destination_port = convert_8_to_16(&packet[36..38]);
    // println!("Destination Port: {}", destination_port);
    // let sequence_number = convert_8_to_32(&packet[38..42]);
    // println!("Sequence number: {}", sequence_number);
    // let ack_number = convert_8_to_32(&packet[42..46]);
    // println!("Acknowledgement number: {}", ack_number);
    // let header_flags = convert_8_to_16(&packet[46..48]);
    // let mut header = header_flags & 0xF000;
    // let flags = header_flags & 0x0FFF;
    // header = header >> 12; 
    // println!("Header: {}", header);
    // println!("Flags: {:#06X}", flags);
    // let window = convert_8_to_16(&packet[48..50]);
    // println!("Window: {}", window);
    // let checksum = convert_8_to_16(&packet[50..52]);
    // println!("Checksum: {:#06X}", checksum);
    // let urgent_pointer = convert_8_to_16(&packet[52..54]);
    // println!("Urgent Pointer: {}", urgent_pointer);

    

}
fn get_tcp_payload(packet: &Packet<'_>, packet_info: &mut PacketInfo){
    
}
pub fn load_pcap_packets(path: &str) -> Result<Vec<PacketInfo>, ErrorResponse>{
    let mut packets: Vec<PacketInfo> = Vec::new();
     match Capture::from_file(path){
        Ok(mut cap) => {
            // let mut offset = 0;
            // let mut packet_num = 1;
            while let Ok(packet) = cap.next_packet(){
                let mut packet_info = PacketInfo::default();

                packet_info.packet_size = Some(packet.header.len);
                get_data_link_frame(&packet, &mut packet_info);
                //println!("{:?}", packet_info.data_link);
                get_ipv4_internet_packet(&packet, &mut packet_info);
                get_transport_datagram_tcp(&packet, &mut packet_info);
                get_tcp_payload(&packet, &mut packet_info);
                packets.push(packet_info);
                //println!("{:?}", packets[0].data_link);
            }
            Ok(packets)
        }
        Err(err) => {
            eprintln!("Could not open pcap file: {}", err);
            Err(ErrorResponse {
                code: constants::PCAP_FILE_ERR_CODE.to_string(),
                error: err.to_string()
            })
        }
    }
}
pub fn _print_packets_in_pcap(packets: Vec<PacketInfo>) {
    let mut packet_number = 1;
    for packet in packets{
        println!("Packet number: {}", packet_number);
        println!("{:?}", packet.data_link.as_ref().unwrap());
        println!("{:?}", packet.ip.as_ref().unwrap());
        println!("{:?}", packet.transport.as_ref().unwrap());

        packet_number += 1;
    }
}
pub fn filter_packets(packets: Vec<PacketInfo>, filter: &PacketFilter) -> Vec<PacketInfo>{
    packets
        .iter()
        .filter(|pck| {
            if let Some(src) = &filter.src_ip{
                if *pck.ip.as_ref().unwrap().src != *src {
                    return false;
                }
            }
            if let Some(dst) = &filter.dst_ip {
                if *pck.ip.as_ref().unwrap().dst != *dst { return false; }
            }
            if let Some(port) = filter.src_port {
                if pck.transport.as_ref().unwrap().src_port != port { return false; }
            }
            if let Some(port) = filter.dst_port {
                if pck.transport.as_ref().unwrap().dst_port != port { return false; }
            }
            true
        })
        .cloned()
        .collect()
}

pub fn router() -> Router{
    Router::new()
        .route("/", get(endpoints::get_pcap_packets))
        .route("/filter", get(get_filter_packets))
}


