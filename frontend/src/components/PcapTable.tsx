import {MaterialReactTable,type MRT_ColumnDef} from "material-react-table";
import { useState } from "react";
import type { PacketInfo } from "../models/packets";
import { fetch_filtered_data } from "../api/api";
import { generate_columns, generate_table_data } from "../utility/utilities";
import "../App.css"

function PcapTable({ props }: any) {
  const [src_ip, setSrcIp] = useState("");
  const [dst_ip, setDstIp] = useState("");
  const [src_port, setSrcPort] = useState(0);
  const [dst_port, setDstPort] = useState(0);

  const handleFetchFilterData = async () => {
    try {
      props.setLoading(true);
      const data: [PacketInfo] = await fetch_filtered_data({
        src_ip,
        dst_ip,
        src_port,
        dst_port,
      });
      props.setData(data);
      console.log(data);
      props.setLoading(false);
    } catch (err) {
      console.log(err);
    }
  };

  const tableData = generate_table_data(props.data);
  const columns: MRT_ColumnDef<any>[] = generate_columns();
  return (
    <div className="text-center mb-4" style={{ width: "80%", margin: "0 auto" }}>
      <div className="row mb-3">
        <div className="col-12">
          <h1 className="page-title">Data from PCAP API</h1>
        </div>
      </div>

      <div className="row mb-3">
        <div className="col-md-3 mb-2">
          <label className="form-label">SRC IP</label>
          <input
            type="text"
            className="form-control"
            value={src_ip}
            onChange={(e) => setSrcIp(e.target.value)}
          />
        </div>

        <div className="col-md-3 mb-2">
          <label className="form-label">DST IP</label>
          <input
            type="text"
            className="form-control"
            value={dst_ip}
            onChange={(e) => setDstIp(e.target.value)}
          />
        </div>

        <div className="col-md-3 mb-2">
          <label className="form-label">SRC PORT</label>
          <input
            type="text"
            className="form-control"
            onChange={(e) => setSrcPort(Number(e.target.value))}
          />
        </div>

        <div className="col-md-3 mb-2">
          <label className="form-label">DST PORT</label>
          <input
            type="text"
            className="form-control"
            onChange={(e) => setDstPort(Number(e.target.value))}
          />
        </div>

        <div className="col-12 mt-2">
          <button className="cool-btn" onClick={handleFetchFilterData}>
            Filter
          </button>
        </div>
      </div>
      <MaterialReactTable
        enableExpanding
        enableExpandAll
        columns={columns}
        data={tableData}
        state={{ isLoading: props.loading }}
        enableGlobalFilter={false}
      />
    </div>
  );
}
export default PcapTable;
