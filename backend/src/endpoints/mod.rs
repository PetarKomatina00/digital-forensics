use crate::{constants, models::PacketInfo, models::PacketFilter};
use crate::repository::{self, filter_packets};
use axum::{http::StatusCode, Json, extract::Query};

pub async fn get_pcap_packets() -> Result<Json<Vec<PacketInfo>>, (StatusCode, String)>{
    let packets = repository::load_pcap_packets(constants::PCAP_FILE);
    match packets{
        Ok(vec_packets) => {
            Ok(Json(vec_packets))
        },
        Err(error) => {
            Err((StatusCode::NOT_FOUND, format!("{}: {}", error.code, error.error)))
        }
    }
}
pub async fn get_filter_packets(Query(filter): Query<PacketFilter>) -> Json<Vec<PacketInfo>>{
    let packets = repository::load_pcap_packets(constants::PCAP_FILE);
    println!("Filter: {:?}", filter);
    match packets{
        Ok(vec_packets) => {
            let filtered_packets = filter_packets(vec_packets, &filter);
            return Json(filtered_packets);
        },
        Err(_error) => {
            //Err((StatusCode::NOT_FOUND, format!("{}: {}", error.code, error.error)))
        }
    }

    Json([PacketInfo::default()].to_vec())
}