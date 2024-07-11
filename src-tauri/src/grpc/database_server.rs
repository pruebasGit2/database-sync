use std::{thread, time::Duration};

use crate::proto::{self, Connection, Databases, GetScriptsRequest, Script};

use proto::database_server::Database;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::database::database;

#[derive(Debug, Default)]
pub struct DatabaseService;

impl DatabaseService {
    pub fn new() -> Self {
        DatabaseService { }
    }
}

#[tonic::async_trait]
impl Database for DatabaseService {

    type GetScriptsStream = ReceiverStream<Result<Script, Status>>;

    async fn get_databases(
        &self,
        _request: Request<Connection>
    ) -> Result<Response<Databases>, Status> {

        let cstr = _request.get_ref().connection_string.clone();

        let dbs = match database::Database::get_all(cstr.as_str()).await {
            Ok(dbs) => dbs,
            Err(err) => return Err(Status::new(tonic::Code::Unknown, err)),
        };

        Ok(Response::new(Databases { database: dbs.into_iter().map(|x| x.name).collect() }))        
    }
    
    async fn get_scripts(
        &self,
        _request: Request<GetScriptsRequest>,
    ) -> Result<Response<Self::GetScriptsStream>, Status> {
        let (tx, rx) = mpsc::channel::<Result<Script, Status>>(2);
        
        tokio::spawn(async move {
            for i in 0..10 {
                if let Err(err) = tx.send(Ok(Script { script: format!("script #{}", i), database: "".to_string() })).await {
                    eprintln!("[grpc-server]: Error sending script reply: {:?}", err);
                }
                thread::sleep(Duration::from_millis(1000));
            }
        });

        Ok(Response::new(Self::GetScriptsStream::new(rx)))
    }
}
