use tonic::{
    transport::Server,
    Request,
    Response,
    Status
};

use regex::Regex;

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
        let reg: Regex = Regex::new("^[0-9]+$").unwrap();
        let data = request.into_inner();
        let pid: String = data.id;
        let a: i32 = data.a;
        let b: i32 = data.b;

        if !reg.is_match(&pid) && !pid.eq("exit") {
            // panic!("Invalid PID");
            return Err(Status::invalid_argument("Invalid PID"));
        }

        if a == -1 && b == -1 {
            println!();
            println!("Bye");
            exit(0);
        }

        println!("- Client[{}]", pid);

        let r = match a.checked_mul(b) {
            Some(val) => val,
            None => return Err(Status::invalid_argument("Invalid Parameter")),
        };

        let response = GetDataResponse {
            res: r
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
