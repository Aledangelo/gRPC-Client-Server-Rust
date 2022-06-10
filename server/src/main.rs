use tonic::{
    transport::Server,
    Request,
    Response,
    Status
};

use servente::servente_server::{
    Servente,
    ServenteServer
};

use servente::{
    GetDataRequest,
    GetDataResponse
};

use std::net::SocketAddr;
use std::process::exit;

mod servente {
    include!("servente.rs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("greeter_descriptor");
}

#[derive(Default)]
pub struct ServernteImpl {}

#[tonic::async_trait]
impl Servente for ServernteImpl {
    async fn get_data(&self, request: Request<GetDataRequest>) -> Result<Response<GetDataResponse>, Status> {
        print!("Request from {:?} ", request.remote_addr());
        let data = request.into_inner();
        let pid: String = data.id;
        let a: i32 = data.a;
        let b: i32 = data.b;

        if a == -1 && b == -1 {
            println!();
            println!("Bye");
            exit(0);
        }

        println!("- Client[{}]", pid);

        let response = GetDataResponse {
            res: a*b
        };
        return Ok(Response::new(response));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "0.0.0.0:50051".parse().unwrap();
    let servente = ServernteImpl::default();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(servente::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    println!("Server listening on: {:?}", addr);
    Server::builder().add_service(ServenteServer::new(servente)).add_service(reflection_service).serve(addr).await?;
    
    Ok(())
}
