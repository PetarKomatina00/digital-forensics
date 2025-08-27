export interface PacketInfo {
  packet_size? : number,
  data_link?: DataLinkFrame;
  ip?: IpPacketV4;
  transport?: Datagram;
}

export interface DataLinkFrame {
  src_mac: string;
  dst_mac: string;
  ethertype: number;
}

export interface IpPacketV4 {
  src: string;
  dst: string;
  protocol: number;
  ttl: number;
}

export interface Datagram{
    src_port: number;
    dst_port: number;
    seq: number;
    ack: number;
    flags: number;
  };

export interface ErrorResponse {
  code: string;
  error: string;
}

export type FilterParams = {
  src_ip? : string,
  dst_ip? : string, 
  src_port? : number,
  dst_port? : number
}
export type SubRow = {
  type: "DataLink" | "Ip" | "Transport";
  data: Record<string, any>;
};