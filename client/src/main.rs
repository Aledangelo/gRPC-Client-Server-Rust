use servente::servente_client::ServenteClient;
use servente::GetDataRequest;
use tonic::Request;
use std::process::id;
use std::{string::String};
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
        let n: u64 = rng.gen_range(100..1000);
        tokio::time::sleep(tokio::time::Duration::from_millis(n)).await;
        let pid: String = id().to_string();
        let req_a: i32 = rng.gen_range(0..100);
        let req_b: i32 = rng.gen_range(0..100);
        print!("Sending => a: {}, b:{} - ", req_a, req_b);
        
        let request = Request::new(GetDataRequest {
            id: pid.into(),
            a: req_a.into(),
            b: req_b.into()
        });

        let response = client.get_data(request).await?;

        println!("Response => {}", response.into_inner().res);
    }
    
    Ok(())
}
