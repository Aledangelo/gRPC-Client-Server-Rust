use std::process::{Command, Stdio, exit};
use std::io::Read;
use servente::servente_client::ServenteClient;
use servente::GetDataRequest;
use tonic::Request;

mod servente {
    include!("../../server/src/servente.rs");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Master");

    let mut handler = vec![];
    
    for _ in 0..3 {
        let out = match Command::new("cargo").arg("run").stdout(Stdio::piped()).current_dir("../client").spawn() {
            Err(why) => panic!("{}", why),
            Ok(out) => out
        };

        handler.push(out);
    }

    for o in handler {
        let mut s = String::new();
        match o.stdout.unwrap().read_to_string(&mut s) {
            Ok(_) => println!("{}", s),
            Err(why) => panic!("{}", why)
        }
    }
    
    
    let mut client = ServenteClient::connect("http://localhost:50051").await?;
    let req_a: i32 = -1;
    let req_b: i32 = -1;
    let req = Request::new(GetDataRequest {
        id: String::from("exit"),
        a: req_a.into(),
        b: req_b.into()
    });

    client.get_data(req).await?;
    exit(0);
    // Ok(())
}