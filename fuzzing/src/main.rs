#[macro_use]
extern crate afl;
use std::{process::exit, str::FromStr};
use servente::{
    GetDataRequest,
    GetDataResponse
};
use tonic::{
    Request,
    Response,
    Status
};

mod servente {
    include!("servente.rs");
}

pub fn get_data(request: Request<GetDataRequest>) -> Result<Response<GetDataResponse>, Status> {
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

use std::str::Split;

fn main() {
    
    fuzz!(|data: &[u8]| {
        let datas = std::str::from_utf8(data).unwrap();
        let data_split: Split<&str> = datas.split(":");
        let mut data_split_clone = data_split.clone();
        if data_split.into_iter().count() == 3 {
            let req = Request::new(GetDataRequest {
                id: data_split_clone.next().unwrap().to_string(),
                a: FromStr::from_str(data_split_clone.next().unwrap()).unwrap(),
                b: FromStr::from_str(data_split_clone.next().unwrap()).unwrap()
            });
            let _ = get_data(req);
        }
    });
}
