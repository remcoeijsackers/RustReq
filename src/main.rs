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
    let resp = reqwest::blocking::get(Cli::into_url(Cli::from_args()))?
        .json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
}
