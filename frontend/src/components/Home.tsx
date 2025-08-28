import { useEffect, useState } from "react";
import type { PacketInfo } from "../models/packets";
import { fetch_all_data } from "../api/api";
import PcapTable from "./PcapTable";
import { Link, Route, Routes } from "react-router-dom";
import { Histogram } from "./Histogram";

export function Home() {
  const [data, setData] = useState<PacketInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [packetAppID, setPacketAppID] = useState<number>(-1);
  const sleep = (ms: number) =>
    new Promise((resolve) => setTimeout(resolve, ms));

  const dataState = {
    data,
    setData,
    loading,
    setLoading,
    error,
    setError,
  };

  useEffect(() => {
    async function handle_fetch_all_data() {
      setLoading(true);
      const data: [PacketInfo] = await fetch_all_data();
      
      setTimeout(() => {
        setLoading(false);
      }, 2000);
      
      setData(data);
    }
    handle_fetch_all_data();
  }, []);

  useEffect(() => {
    data.map((packet : PacketInfo, index: number) => {
      if (packet.http?.app_id !== undefined){
        setPacketAppID(index + 1)
      }
    })
  }, [data])
  if (loading) {
    return (
      <div className="loader-container">
        <div className="loader"></div>
      </div>
    );
  } else if (data.length > 0) {
    if (data) {
      return (
        <>
          <nav className="d-flex justify-content-center mb-3 mt-3">
            <Link to="/histogram" className="cool-btn" state={{ data }}>
              Go to Histogram
            </Link>
          </nav>
          <PcapTable props={dataState} packetAppID = {packetAppID}/>
        </>
      );
    }
  }
}
