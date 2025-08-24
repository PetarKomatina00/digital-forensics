use axum::{
    routing::get,
    Router,
};
use pcap::Capture;


mod models;
mod repository;
mod endpoints;
mod utility;
mod constants;
use axum;
#[tokio::main]
async fn main() {
    
    // let app: Router = Router::new().route("/", get(|| async { "Hello, World!" }));

    // let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();

    let packets: Vec<models::PacketInfo> = repository::load_pcap_packets(constants::PCAP_FILE);
    repository::print_packets_in_pcap(packets);

   
}
