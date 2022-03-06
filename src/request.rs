use std::io::Read;

pub struct resobj {
    pub status: String,
    pub headers: String,
    pub resbody: String,
}

pub fn build_resobj(status: String, headers: String, resbody: String) -> resobj {
    resobj {
        status : status,
        headers : headers,
        resbody: resbody
    }
}

pub fn grequest(pattern:String) -> Result<resobj, Box<dyn std::error::Error>> {
    let pattern = pattern;
    let mut res = reqwest::blocking::get(pattern)?;
    let mut body = String::new();
    let mut stcode = String::new();
    let mut hdr = String::new();
    
    res.read_to_string(&mut body)?;
    res.read_to_string(&mut stcode)?;
    res.read_to_string(&mut hdr)?;
    
    let rsp = build_resobj(stcode, hdr, body);

    Ok(rsp)
}

pub fn prequest(pattern:String) -> Result<resobj, Box<dyn std::error::Error>> {
    let pattern = pattern;
    let mut res = reqwest::blocking::get(pattern)?;
    let mut body = String::new();
    let mut stcode = String::new();
    let mut hdr = String::new();
    
    res.read_to_string(&mut body)?;
    res.read_to_string(&mut stcode)?;
    res.read_to_string(&mut hdr)?;
    
    let rsp = build_resobj(stcode, hdr, body);

    Ok(rsp)
}

pub fn mrequest() -> Result<resobj, Box<dyn std::error::Error>> {
    let st = String::from("unkown request method");
    let hd = String::from("");
    let bd = String::from("");
    let rsp = build_resobj(st, hd, bd);

    Ok(rsp)
}