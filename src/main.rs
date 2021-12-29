use std::io::Read;
mod request;
use request::mrequest;
use request::prequest;
use request::resobj;

pub fn rqesthandler(reqtpe: String, pattern: String) -> resobj {
    let dt: resobj = mrequest(pattern).unwrap();
    println!("Status: {}", dt.status);
    println!("Headers:\n{:#?}", dt.headers);
    println!("Body:\n{}", dt.resbody);
    return dt
}

fn main() {
    let reqtype = std::env::args().nth(1).expect("no request type / url given");
    let pattern = std::env::args().nth(2).expect("no request type / url given");
    let get = "get";
    let post = "post";
    let outcome = match reqtype{
        get => rqesthandler(get,pattern),
        post => rqesthandler(post,pattern),
    };
    
}
