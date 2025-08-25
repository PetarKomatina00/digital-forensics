import "./App.css";
import { useEffect, useState } from "react";
import {
  MaterialReactTable,
  useMaterialReactTable,
  type MRT_ColumnDef,
} from "material-react-table";
import type { PacketInfo } from "./models/packets.js";

type SubRow = {
  type: "DataLink" | "Ip" | "Transport";
  data: Record<string, any>;
};
function App() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(true);
  useEffect(() => {
    fetch("http://127.0.0.1:4000")
      .then((res) => res.json())
      .then((result) => setData(result))
      .finally(() => setLoading(false));
  }, []);

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
      subRows: subRows.map((s) => ({
        type: s.type,
        data: s.data,
        subRows: Object.entries(s.data).map(([key, value]) => ({
          key,
          value,
        })),
      })),
    };
  });
  const columns: MRT_ColumnDef<any>[] = [
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
  ];
  if (loading) {
    <div className="container">
      <div className="row">
        <div className="col-12 col-md-6 col-lg-4">
          <div className="loader">Hi</div>
        </div>
      </div>
    </div>;
  } else if (data.length > 0) {
    if (data) {
      return (
        <div className="container">
          <div className="row">
            <div className="col-12 col-md-6 col-lg-4">
              <h1>Data from PCAP API</h1>
            </div>
            <div className="row">
              <MaterialReactTable
                enableExpanding
                enableExpandAll
                columns={columns}
                data={tableData}
                state={{ isLoading: loading }}
                enableGlobalFilter={false}  
              />
            </div>
          </div>

          <div></div>
        </div>
      );
    }
  }
}

export default App;
