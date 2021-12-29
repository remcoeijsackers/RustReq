use std::io::Read;


pub fn mrequest(pattern:String) -> Result<(), Box<dyn std::error::Error>> {
    let pattern = pattern;
    let mut res = reqwest::blocking::get(pattern)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}

pub fn prequest(pattern:String) -> Result<(), Box<dyn std::error::Error>> {
    let pattern = pattern;
    let mut res = reqwest::blocking::get(pattern)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}
