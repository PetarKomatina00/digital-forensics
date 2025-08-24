use axum::{routing::get, Router};
use pcap::{Capture, Packet};
use crate::models::{DataLinkFrame, Datagram, IpPacketV4, PacketInfo};
use crate::utility;
use std::io::{self, Write};

pub fn get_data_link_frame(packet: &Packet, packet_info: &mut PacketInfo) {
    
    let data_link_frame = DataLinkFrame {
        src_mac : utility::mac_to_string(&packet[0..6]),
        dst_mac : utility::mac_to_string(&packet[6..12]),
        ethertype : utility::convert_8_to_16(&packet[12..14])
    };
    packet_info.data_link = Some(data_link_frame);
    
}
pub fn get_ipv4_internet_packet(packet: &Packet, packet_info: &mut PacketInfo){
    let source_arr = &packet[26..30];
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
pub fn get_transport_datagram_tcp(packet: &Packet, packet_info: &mut PacketInfo){
    if packet_info.ip.as_ref().unwrap().protocol == 6{
        // TCP
        let datagram = Datagram::Tcp 
        { 
            src_port: utility::convert_8_to_16(&packet[34..36]), 
            dst_port: utility::convert_8_to_16(&packet[38..42]), 
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
pub fn load_pcap_packets(path: &str) -> Vec<PacketInfo>{
    let mut packets: Vec<PacketInfo> = Vec::new();
     match Capture::from_file(path){
        Ok(mut cap) => {
            // let mut offset = 0;
            // let mut packet_num = 1;
            while let Ok(packet) = cap.next_packet(){
                let mut packet_info = PacketInfo::default();
                get_data_link_frame(&packet, &mut packet_info);
                //println!("{:?}", packet_info.data_link);
                get_ipv4_internet_packet(&packet, &mut packet_info);
                get_transport_datagram_tcp(&packet, &mut packet_info);

                packets.push(packet_info);
                //println!("{:?}", packets[0].data_link);
            }
        }
        Err(err) => {
            eprintln!("Could not open pcap file: {}", err);
        }
    }
    packets
}
pub fn print_packets_in_pcap(packets: Vec<PacketInfo>) {
    let mut packet_number = 1;
    for packet in packets{
        println!("Packet number: {}", packet_number);
        println!("{:?}", packet.data_link.as_ref().unwrap());
        println!("{:?}", packet.ip.as_ref().unwrap());
        println!("{:?}", packet.transport.as_ref().unwrap());

        packet_number += 1;
    }
}
// pub fn router() -> Router{
//     Router::new 
//         .route("/frame", get(get_data_link_frame))
// }


