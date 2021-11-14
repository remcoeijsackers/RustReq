use std::collections::HashMap;
use structopt::StructOpt;
use std::error::Error;
use url::{Url, ParseError};

trait IntoUrl {
    fn into_url(self) -> String;
}

#[derive(StructOpt)]
#[derive(Debug)]
struct Cli {
    /// The pattern to look for
    pub url: String,
}

impl IntoUrl for Cli {
    fn into_url(self) -> String
    {self.url}
}

#[derive(StructOpt)]
struct Rp {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    response: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let ur = Cli::into_url(args);
    let resp = reqwest::blocking::get(ur)?
        .json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
}
