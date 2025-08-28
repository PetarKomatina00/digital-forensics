use std::fs;
use std::io::{self, Write};

use crate::endpoints::{export_pdf, get_filter_packets};
use crate::models::{
    DataLinkFrame, Datagram, ErrorResponse, IpPacketV4, PacketFilter, PacketInfo, WeatherAppId,
};
use crate::{constants, endpoints, utility};
use axum::routing::post;
use axum::{http, Json};
use axum::{Router, routing::get};
use bytes::Bytes;
use genpdf::elements::{LinearLayout, Paragraph};
use genpdf::{elements, fonts, Document};
use pcap::{Capture, Packet};
use std::io::Cursor;
fn get_weather_appid(packet_info: &mut PacketInfo) {
    let http = packet_info
        .transport
        .as_ref()
        .unwrap()
        .payload
        .as_ref()
        .unwrap();

    let marker = "appid=";
    let app_id: Option<usize> = http.find(marker);
    if app_id.is_some() {
        let start_app_id = app_id.unwrap() + marker.len();
        let rest = &http[start_app_id..];
        let end_app_id = rest.find('&').unwrap_or(rest.len());
        let weather_app_id = WeatherAppId {
            app_id: rest[..end_app_id].to_string(),
        };
        packet_info.http = Some(weather_app_id);
    }
}
fn check_if_payload_exists(ip_header_len: u8, ip_total_len: u16, transport_header_len: u8) -> bool {
    //println!("IP header {} TCP Header {} IP total {}", ip_header_len, transport_header_len, ip_total_len);
    //These are valuated in words. 1 word is 4 Bytes. Need to compare bytes
    let ip_header_len_bytes = (ip_header_len as u16);
    let tcp_header_len_bytes = (transport_header_len as u16);

    if ip_header_len_bytes + tcp_header_len_bytes >= ip_total_len {
        return false;
    }
    let payload_len = ip_total_len - (ip_header_len_bytes + tcp_header_len_bytes);
    payload_len > 0
}
pub fn get_data_link_frame(packet: &Packet<'_>, packet_info: &mut PacketInfo) {
    let data_link_frame = DataLinkFrame {
        src_mac: utility::mac_to_string(&packet[0..6]),
        dst_mac: utility::mac_to_string(&packet[6..12]),
        ethertype: utility::convert_8_to_16(&packet[12..14]),
    };
    packet_info.data_link = Some(data_link_frame);
}
pub fn get_ipv4_internet_packet(packet: &Packet<'_>, packet_info: &mut PacketInfo) {
    let ip_packet = IpPacketV4 {
        src: utility::ip_addr_to_string(&packet[26..30]),
        dst: utility::ip_addr_to_string(&packet[30..34]),
        protocol: packet[23],
        ttl: packet[22],
        header_len: packet[14] & 0x0F,
        total_len: utility::convert_8_to_16(&packet[16..18]),
    };
    packet_info.ip = Some(ip_packet);
}
pub fn get_transport_datagram_tcp(packet: &Packet<'_>, packet_info: &mut PacketInfo) {
    if packet_info.ip.as_ref().unwrap().protocol == 6 {
        let mut datagram = Datagram {
            src_port: utility::convert_8_to_16(&packet[34..36]),
            dst_port: utility::convert_8_to_16(&packet[36..38]),
            seq: utility::convert_8_to_32(&packet[38..42]),
            ack: utility::convert_8_to_32(&packet[42..46]),
            header_len: utility::get_header_len(&packet[46]),
            flags: utility::convert_8_to_16(&packet[46..48]) & 0x0FFF,
            payload: None,
        };
        datagram.payload = get_tcp_payload(&packet, packet_info, datagram.header_len);
        packet_info.transport = Some(datagram);
    }
}
fn get_tcp_payload(
    packet: &Packet<'_>,
    packet_info: &mut PacketInfo,
    transport_header_len: u8,
) -> Option<String> {
    let ip_header_len = packet_info.ip.as_ref().unwrap().header_len;
    let ip_total_len = packet_info.ip.as_ref().unwrap().total_len;

    if !check_if_payload_exists(ip_header_len, ip_total_len, transport_header_len) {
        return None;
    }
    let ip_header_len = (ip_header_len as usize) * 4;
    let tcp_header_len = (transport_header_len as usize) * 4;
    let payload_start = 14 + ip_header_len + tcp_header_len;

    let payload = &packet[payload_start..];
    if !payload.is_empty() {
        if let Ok(text) = std::str::from_utf8(payload) {
            return Some(text.to_string());
        }
    }

    None
}
pub fn load_pcap_packets(path: &str) -> Result<Vec<PacketInfo>, ErrorResponse> {
    let mut packets: Vec<PacketInfo> = Vec::new();
    match Capture::from_file(path) {
        Ok(mut cap) => {
            // let mut offset = 0;
            let mut packet_num = 1;
            while let Ok(packet) = cap.next_packet() {
                let mut packet_info = PacketInfo::default();

                packet_info.packet_size = Some(packet.header.len);
                get_data_link_frame(&packet, &mut packet_info);
                get_ipv4_internet_packet(&packet, &mut packet_info);
                get_transport_datagram_tcp(&packet, &mut packet_info);
                if packet_info.transport.as_ref().unwrap().payload.is_some() {
                    get_weather_appid(&mut packet_info);
                } else {
                    packet_info.http = None
                }
                packets.push(packet_info);
                packet_num += 1;
            }
            Ok(packets)
        }
        Err(err) => {
            eprintln!("Could not open pcap file: {}", err);
            Err(ErrorResponse {
                code: constants::PCAP_FILE_ERR_CODE.to_string(),
                error: err.to_string(),
            })
        }
    }
}
pub fn _print_packets_in_pcap(packets: Vec<PacketInfo>) {
    let mut packet_number = 1;
    for packet in packets {

        packet_number += 1;
    }
}
pub fn filter_packets(packets: Vec<PacketInfo>, filter: &PacketFilter) -> Vec<PacketInfo> {
    packets
        .iter()
        .filter(|pck| {
            if let Some(src) = &filter.src_ip {
                if *pck.ip.as_ref().unwrap().src != *src {
                    return false;
                }
            }
            if let Some(dst) = &filter.dst_ip {
                if *pck.ip.as_ref().unwrap().dst != *dst {
                    return false;
                }
            }
            if let Some(port) = filter.src_port {
                if pck.transport.as_ref().unwrap().src_port != port {
                    return false;
                }
            }
            if let Some(port) = filter.dst_port {
                if pck.transport.as_ref().unwrap().dst_port != port {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect()
}

pub fn generate_pdf_report(packets: Vec<PacketInfo>) -> Vec<u8> {
    let font_family = genpdf::fonts::from_files("src/assets/fonts", "DejaVuSans", None)
        .expect("Failed to load font family");

    let mut doc = genpdf::Document::new(font_family);

    for (index, packet) in packets.iter().enumerate(){
        let text = format!("Packet number: {index}");
        let frame = format!("{:?}", packet.data_link);
        let ip = format!("{:?}", packet.ip);
        let datagram = format!("{:?}", packet.transport);
        let http = format!("{:?}", packet.http);

        let mut layout = LinearLayout::vertical();
        layout.push(Paragraph::new(text));
        layout.push(Paragraph::new(frame));
        layout.push(Paragraph::new(ip));
        layout.push(Paragraph::new(datagram));
        layout.push(Paragraph::new(http));

        doc.push(layout);
    }

    let mut buffer = std::io::Cursor::new(Vec::new());
    doc.render(&mut buffer).expect("Failed to render PDF");
    buffer.into_inner()
}
pub fn router() -> Router {
    Router::new()
        .route("/", get(endpoints::get_pcap_packets))
        .route("/filter", get(get_filter_packets))
        .route("/export", post(export_pdf))
}
