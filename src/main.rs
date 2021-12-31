mod request;
use request::grequest;
use request::prequest;
use request::mrequest;
use request::resobj;
mod util;
use util::reqobj;
use util::build_reqobj;
 
pub fn rqesthandler(req: reqobj) -> Result<resobj,()> {
    let dt: resobj;
    let clone = req.clone();

    match req.rtype.as_str(){
        "get" => dt = grequest(req.rpattern).unwrap(),
        "post" => dt = prequest(clone.rpattern).unwrap(),
        _ => dt = mrequest().unwrap(),
    }

    Ok(dt)
}

fn main() {
    let reqtype = String::from(std::env::args().nth(1).expect("no request type / url given"));
    let pattern = std::env::args().nth(2).expect("no request type / url given");

    let req = build_reqobj(pattern, reqtype);
    let outcome = rqesthandler(req).unwrap();

    println!("Status: {}", outcome.status);
    if outcome.headers != "" {
        println!("Headers:\n{:#?}", outcome.headers);
        println!("Body:\n{}", outcome.resbody);
    }
    
}
