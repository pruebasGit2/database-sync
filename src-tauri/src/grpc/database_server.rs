use crate::{compare::database::Db, database::esquema::Esquema, proto::{self, Connection, Databases, DownloadScriptsRequest, GetScriptsRequest, Script}, utils::error_utils::DbError};

use proto::database_server::Database;
use tokio::{fs::File, io::AsyncWriteExt, sync::mpsc};
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

        let dbs = match database::DatabaseSql::get_all(cstr.as_str()).await {
            Ok(dbs) => dbs,
            Err(err) => return Err(Status::new(tonic::Code::Unknown, err)),
        };

        Ok(Response::new(Databases { database: dbs.into_iter().map(|x| x.name).collect() }))        
    }
    
    async fn get_scripts(
        &self,
        _request: Request<GetScriptsRequest>,
    ) -> Result<Response<Self::GetScriptsStream>, Status> {

        let cstr = _request.get_ref().connection_string.to_owned();
        let databases = _request.get_ref().databases.to_owned();
        let databases_base = _request.get_ref().databases_base.to_owned();

        let (tx, rx) = mpsc::channel::<Result<Script, Status>>(2);
        
        tokio::spawn(async move {

            for db_base in databases_base {
                let esquemas_db_base_res = Esquema::get_all(cstr.as_ref(), db_base.as_ref()).await;
                let esquemas_db_base = match esquemas_db_base_res {
                    Ok(esquemas) => esquemas,
                    Err(err) => {
                        eprintln!("[grpc-server]: Cannot get esquemas for db '{}': {:?}", db_base, err);
                        continue
                    }
                };

                let mut database_base = Db::new(db_base.clone(), esquemas_db_base);

                for db in &databases {
                    if db_base.eq(db) { 
                        continue;
                    }

                    let esquemas_db_res = Esquema::get_all(cstr.as_ref(), db).await;
                    let esquemas_db = match esquemas_db_res {
                        Ok(esquemas) => esquemas,
                        Err(err) => {
                            eprintln!("[grpc-server]: Cannot get esquemas for db '{}': {:?}", db, err);
                            Vec::new()
                        }
                    };

                    let mut database_base_other = Db::new(db.clone(), esquemas_db); 

                    for script in database_base.compare(&mut database_base_other) {
                        if let Err(_) = tx.send(Ok(Script { script, database: db.clone() })).await {
                            eprintln!("[grpc-server]: Error sending script");
                        }
                    }
                }
            }
        });

        Ok(Response::new(Self::GetScriptsStream::new(rx)))
    }

    async fn download_scripts(
        &self,
        _request: Request<DownloadScriptsRequest>
    ) -> Result<Response<()>, Status> {
        let mut path = _request.get_ref().path.to_owned(); 
        let scripts = _request.get_ref().scripts.to_owned();

        path.push_str("\\sync_database.sql");

        let mut file = match File::create(path).await {
            Ok(file) => file,
            Err(err) => return Err(Status::new(tonic::Code::Unknown, DbError::Io(err))),
        };

        match file.write_all(scripts.join("\r\n").as_bytes()).await {
            Ok(_) => Ok(Response::new(())),
            Err(err) => Err(Status::new(tonic::Code::Unknown, DbError::Io(err))),
        }
    }

}
