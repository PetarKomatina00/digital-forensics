import {
  LineChart,
  Line,
  CartesianGrid,
  XAxis,
  YAxis,
  Legend,
  Tooltip,
} from "recharts";
import { Link, useLocation } from "react-router-dom";
import type { PacketInfo } from "../models/packets";
import { useEffect, useState } from "react";
// const data = [
//   { name: "Page A", uv: 400, pv: 2400, amt: 2400 },
//   { name: "Page B", uv: 300, pv: 2400, amt: 2400 },
// ];
export function Histogram() {
  const [formattedData, setFormattedData] = useState<any[]>([]);
  const location = useLocation();
  const { data } = (location.state as { data: PacketInfo[] }) || { data: [] };

  useEffect(() => {
    if (data && data.length > 0) {
      const temp = data.map((packet, i) => ({
        packet_size: packet.packet_size,
        index: i + 1,
      }));
      setFormattedData(temp);
      console.log(formattedData);
    }
  }, [data]);

  return (
    <div className="">
      <Link to={"/"} className="btn btn-primary me-2 mt-3" style={{justifyContent : "center"}}>Home</Link>
      <div className="d-flex justify-content-center mt-5">
        <LineChart
          width={900}
          height={600}
          data={formattedData}
          margin={{ top: 5, right: 20, bottom: 5, left: 0 }}
        >
          <CartesianGrid stroke="#aaa" strokeDasharray="5 5" />
          <Line
            type="monotone"
            dataKey="packet_size"
            stroke="purple"
            strokeWidth={2}
            name="Petar Komatina 2025"
          />
          <XAxis
            dataKey="index"
            label={{ value: "Packet number", position: "bottom", offset: 10 }}
          />
          <YAxis
            width="auto"
            label={{ value: "Packet size", position: "insideLeft", angle: -90 }}
          />
          <Legend align="right" />
          <Tooltip formatter={(value, name, props) => [`Bytes: ${value}`]} />
        </LineChart>
      </div>
    </div>
  );
}
