use std::collections::HashMap;
use structopt::StructOpt;

trait IntoUrl {
    fn into_url(self) -> String;
}

#[derive(StructOpt)]
#[derive(Debug)]
struct Cli {
    pub url: String,
}

impl IntoUrl for Cli {
    fn into_url(self) -> String
    {self.url}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let ur = Cli::into_url(args);
    let resp = reqwest::blocking::get(ur)?
        .json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
}
