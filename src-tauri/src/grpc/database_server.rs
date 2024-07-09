use crate::proto::{self, Databases, EmptyRequest};

use proto::database_server::Database;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct DatabaseService;

impl DatabaseService {
    pub fn new() -> Self {
        DatabaseService { }
    }
}

#[tonic::async_trait]
impl Database for DatabaseService {

    async fn get_databases(
        &self,
        _request: Request<EmptyRequest>
    ) -> Result<Response<Databases>, Status> {
        Ok(Response::new(Databases { database: vec![String::from("0001"), String::from("0002")] }))        
    }

}
