use std::io::Read;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pattern = std::env::args().nth(1).expect("no website given");
    let mut res = reqwest::blocking::get(pattern)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}
