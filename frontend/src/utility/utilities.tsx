import type { MRT_ColumnDef } from "material-react-table";
import type { PacketInfo, SubRow } from "../models/packets";

export function generate_table_data(data: [PacketInfo], packetAppId: number): any{
  const tableData = data.map((packet: PacketInfo, index: number) => {
    const subRows: SubRow[] = [];
    if (packet.data_link) {
      subRows.push({ type: "DataLink", data: packet.data_link });
    }
    if (packet.ip) {
      subRows.push({ type: "Ip", data: packet.ip });
    }
    if (packet.transport) {
      subRows.push({ type: "Transport", data: packet.transport });
    }
    return {
      id: index + 1,
      label: `Packet ${index + 1}`,
      subRows: subRows.map((s) => {
        let entries = Object.entries(s.data);
        if (s.type === "Transport") {
          const allowedKeys = ["src_port", "dst_port", "seq", "ack", "flags", "payload"]
          entries = entries.filter(([key]) => allowedKeys.includes(key))
          if (index + 1 === packetAppId && packet.http?.app_id){
              entries = [["app_id", packet.http.app_id], ...entries];
          }
        }
        
        return {
          type: s.type,
          data: s.data,
          subRows: entries.map(([key, value]) => ({
            key,
            value,
          })),
        };
      }),
    };
  });
  return tableData;
}
export function generate_columns(): MRT_ColumnDef<any>[]{
    return [
    {
      header: "Label",
      accessorKey: "label",
    },
    {
      header: "Type",
      accessorKey: "type",
    },
    {
      header: "Key",
      accessorKey: "key",
    },
    {
      header: "Value",
      accessorKey: "value",
    },
  ]
}