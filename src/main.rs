use std::io::Read;
mod request;
use request::mrequest;
use request::prequest;

fn main() {
    let reqtype = std::env::args().nth(1).expect("no request type / url given");
    let pattern = std::env::args().nth(2).expect("no request type / url given");
    let get = "get";
    let post = "post";
    let outcome = match reqtype{
        get => mrequest(pattern),
        post => prequest(pattern),
    };
    
}
