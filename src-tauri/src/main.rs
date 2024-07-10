// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod grpc;

pub mod proto {
    tonic::include_proto!("database");
}

use grpc::database_server::DatabaseService;
use proto::database_server::DatabaseServer;
use tokio::spawn;
use tonic::transport::Server;

pub async fn run_grpc_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "192.168.10.12:3500".parse().unwrap();

    let sv = DatabaseServer::new(DatabaseService::new());

    println!("[grpc-server]: listening on: {}", addr);

    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(sv))
        .serve(addr)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    spawn(async move {
        println!("START");
        if let Err(e) = run_grpc_server().await {
            eprintln!("Failed to run gRPC server: {}", e);
        }
    });

    tauri::Builder::default()
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
