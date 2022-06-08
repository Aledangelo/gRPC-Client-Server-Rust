use servente::servente_client::ServenteClient;
use servente::GetDataRequest;
use tonic::Request;
use std::string::String;
use rand::{
    thread_rng,
    Rng
};

mod servente {
    include!("../../server/src/servente.rs");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ServenteClient::connect("http://localhost:50051").await?;
    let mut rng = thread_rng();
    for _ in 0..5 {
        let req_a: i32 = rng.gen_range(0..100);
        let req_b: i32 = rng.gen_range(0..100);
        print!("Sending => a: {}, b:{} - ", req_a, req_b);
        
        let request = Request::new(GetDataRequest {
            id: String::from("ciccio").into(),
            a: req_a.into(),
            b: req_b.into()
        });

        let response = client.get_data(request).await?;

        println!("Response => {}", response.into_inner().res);
    }
    
    Ok(())
}
