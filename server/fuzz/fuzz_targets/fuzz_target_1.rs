#![no_main]
use servente::servente_client::ServenteClient;
use servente::servente_server::Servente;
use servente::GetDataRequest;
use tonic::Request;

mod servente {
    include!("../../src/servente.rs");
}

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    // let mut client = ServenteClient::connect("http://localhost:50051").await?;

    let req = Request::new(GetDataRequest {
        id: data[0].into(),
        a: data[1].into(),
        b: data[2].into()
    });
    let response = Servente::get_data(req);
});
