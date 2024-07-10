// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod grpc;

pub mod proto {
    tonic::include_proto!("database");
}

use grpc::database_server::DatabaseService;
use proto::database_server::DatabaseServer;
use tokio::{spawn, sync::oneshot};
use tonic::transport::Server;

pub async fn run_grpc_server(
    shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:777".parse().unwrap();
    let svc = DatabaseServer::new(DatabaseService::new());

    println!("[grpc]: server on [::1]:777");

    Server::builder()
        .add_service(svc)
        .serve_with_shutdown(addr, async {
            shutdown_rx.await.ok();
        })
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let (_, shutdown_rx) = oneshot::channel::<()>();

    spawn(async move {
        println!("START");
        if let Err(e) = run_grpc_server(shutdown_rx).await {
            eprintln!("Failed to run gRPC server: {}", e);
        }
    });

    tauri::Builder::default()
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
