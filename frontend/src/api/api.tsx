import { backend } from "../constants/constants";
import type { FilterParams, PacketInfo } from "../models/packets";
export async function fetch_filtered_data({
  src_ip,
  dst_ip,
  src_port,
  dst_port,
}: FilterParams): Promise<[PacketInfo]> {
  try {
    const params = new URLSearchParams();
    if (src_ip) params.append("src_ip", src_ip);
    if (dst_ip) params.append("dst_ip", dst_ip);
    if (src_port !== 0 && src_port !== undefined)
      params.append("src_port", src_port.toString());
    if (dst_port !== 0 && dst_port !== undefined)
      params.append("dst_port", dst_port.toString());

    const url = `${backend}/filter?${params.toString()}`;

    console.log("URL: {}", url);
    const response = await fetch(url);
    if (!response.ok) {
      throw new Error("Something went wrong");
    }
    return response.json() as Promise<[PacketInfo]>;
  } catch (err) {
    console.log(err);
    throw new Error("Something went wrong");
  }
}

export async function fetch_all_data(): Promise<[PacketInfo]> {
  try {
    const response = await fetch(`${backend}`);
    if (!response.ok) {
      throw new Error("Server error");
    }
    const json = await response.json();
    return json as Promise<[PacketInfo]>;
  } catch (err) {
    console.log(err);
    throw new Error("Something went wrong");
  }
}

export async function fetch_export_pdf(data: [PacketInfo]){
  try{
    const response = await fetch(`${backend}/export`, {
      method : "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(data),
    });

    if (!response.ok){
      throw new Error("Server error");
    }
    console.log(response.body);

    const blob = await response.blob();
    const url = window.URL.createObjectURL(blob);

    const doc = document.createElement("a");
    doc.href = url;
    document.body.appendChild(doc);
    doc.click();
    window.URL.revokeObjectURL(url);
  }
  catch(err){
    console.log(err);
    throw new Error("Something went wrong");
  }
}