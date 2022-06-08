use std::process::{Command, Stdio};
use std::io::Read;

fn main() {
    println!("Master");

    let mut handler = vec![];
    
    for _ in 0..3 {
        let out = match Command::new("cargo").arg("run").stdout(Stdio::piped()).current_dir("../client").spawn() {
            Err(why) => panic!("{}", why),
            Ok(out) => out,
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

    
}