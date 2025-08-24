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

#[derive(Debug, Default, Clone)]
pub struct PacketInfo{
    pub data_link: Option<DataLinkFrame>,
    pub ip: Option<IpPacketV4>,
    pub transport: Option<Datagram>,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DataLinkFrame{
    pub src_mac: String,
    pub dst_mac: String,
    pub ethertype: u16,
}
#[derive(Debug, Default, Clone)]
pub struct IpPacketV4 {
    pub src: String,
    pub dst: String,
    pub protocol: u8,   // e.g. 6 = TCP, 17 = UDP
    pub ttl: u8,
} 
#[derive(Debug, Clone, Serialize)]
pub enum Datagram {
    Tcp {
        src_port: u16,
        dst_port: u16,
        seq: u32,
        ack: u32,
        flags: u16,
    },
    Udp {
        src_port: u16,
        dst_port: u16,
        length: u16,
    },
}
impl Default for Datagram{
    fn default() -> Self {
            Datagram::Tcp {
            src_port: 0,
            dst_port: 0,
            seq: 0,
            ack: 0,
            flags: 0,
        }
    }
}