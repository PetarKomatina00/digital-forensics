import { MaterialReactTable, type MRT_ColumnDef } from "material-react-table";
import { useState } from "react";
import type { PacketInfo } from "../models/packets";
import { fetch_export_pdf, fetch_filtered_data } from "../api/api";
import { generate_columns, generate_table_data } from "../utility/utilities";
import "../App.css";
import { toast } from "react-toastify";
function PcapTable({ props, packetAppID }: any) {
  const [src_ip, setSrcIp] = useState("");
  const [dst_ip, setDstIp] = useState("");
  const [src_port, setSrcPort] = useState(0);
  const [dst_port, setDstPort] = useState(0);

  const [pdfLoading, setPdfLoading] = useState(false);

  const handleExportPDF = async () => {
    console.log("Clicked");

    toast.promise(fetch_export_pdf(props.data), {
      pending: "Generating PDF...",
      success: "PDF Downloaded!",
      error: "Failed to download PDF",
    });
  };
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
      props.setLoading(false);
    } catch (err) {
      console.log(err);
    }
  };

  const tableData = generate_table_data(props.data, packetAppID);
  const columns: MRT_ColumnDef<any>[] = generate_columns();
  return (
    <div
      className="text-center mb-4"
      style={{ width: "80%", margin: "0 auto" }}
    >
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
          <button className="cool-btn ms-5" onClick={handleExportPDF}>
            Export
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
        muiTableBodyRowProps={({ row }) => ({
          sx: {
            backgroundColor:
              row.original.id === packetAppID ? "red" : "inherit",
          },
        })}
      />
    </div>
  );
}
export default PcapTable;
