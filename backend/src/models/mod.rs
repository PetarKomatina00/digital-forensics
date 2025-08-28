use serde::{Serialize, Deserialize};

// struct PacketSummary{
//     frame_number: u32,
//     timestamp: f64,
//     src_ip: String,
//     dst_ip: String, 
//     procotol: String, 
//     src_port: u16,
//     dst_port: u16,
//     length: u32,

//     tcp_flags: String,
//     seq_number: u32,
//     ack_number: u32,

//     http_method: Option<String>,
//     http_host: Option<String>,
//     http_uri: Option<String>,
//     http_status: Option<u16>, 
// }

#[derive(Debug, Default, Clone, Serialize)]
pub struct PacketInfo{
    pub packet_size: Option<u32>,
    pub data_link: Option<DataLinkFrame>,
    pub ip: Option<IpPacketV4>,
    pub transport: Option<Datagram>,
    pub http: Option<WeatherAppId>
}

#[derive(Debug, Default, PartialEq, Clone, Serialize)]
pub struct DataLinkFrame{
    pub src_mac: String,
    pub dst_mac: String,
    pub ethertype: u16,
}
#[derive(Debug, Default, Clone, Serialize)]
pub struct IpPacketV4 {
    pub src: String,
    pub dst: String,
    pub protocol: u8,   // e.g. 6 = TCP, 17 = UDP
    pub ttl: u8,
    pub header_len: u8,
    pub total_len: u16
} 
#[derive(Debug, Clone, Serialize)]
pub struct Datagram {
    pub src_port: u16,
    pub dst_port: u16,
    pub seq: u32,
    pub ack: u32,
    pub flags: u16,
    pub header_len: u8,
    pub payload: Option<String>
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct WeatherAppId{
    pub app_id: String
}
#[derive(Debug)]
pub struct ErrorResponse{
    pub code: String,
    pub error: String, 
}
impl Default for Datagram{
    fn default() -> Self {
        Datagram { src_port: 0, dst_port: 0, seq: 0, ack: 0, flags: 0, header_len: 0, payload: None}
    }
}


#[derive(Debug, Deserialize)]
pub struct PacketFilter{
    pub src_ip: Option<String>,
    pub dst_ip: Option<String>,
    pub src_port: Option<u16>,
    pub dst_port: Option<u16>
}