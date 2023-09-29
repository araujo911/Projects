extern crate base64;
use std::env;
use std::str;

fn help() {
    let help = "
base64_util <options> <string>
    
    options:
        --e = encode string.
        --d = wrap string.
    ";

    println!("{}", help);
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
    }

    let a = args[1].trim();

    match a {
        "--e" => {
            let b64 = base64::encode(&args[2]);
            println!("{}", b64);
        }
        "--d" => {
            let bytes = base64::decode(&args[2]).unwrap();
            println!("{}", str::from_utf8(&bytes).unwrap());
        }
        _ => help(),
    }
}
