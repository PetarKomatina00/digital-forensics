
use crate::utility;
use crate::{constants, models::PacketInfo, models::PacketFilter};
use crate::repository::{self, filter_packets, generate_pdf_report};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{http::StatusCode, Json, extract::Query, http::header};


pub async fn export_pdf(Json(packets): Json<Vec<PacketInfo>>) -> impl IntoResponse{
    let pdf_bytes = generate_pdf_report(packets);

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
    headers.insert(
        header::CONTENT_DISPOSITION,
        r#"attachment; filename="report.pdf""#.parse().unwrap(),
    );
    (StatusCode::OK, headers, pdf_bytes)
}


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