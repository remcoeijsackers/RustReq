use serde::{Deserialize, Serialize};
use serde_json::from_str;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::StatusCode;

#[derive(Clone)]
#[derive(Debug, Serialize, Deserialize)]
pub struct reqobj {
    pub rpattern: String,
    pub rtype: String,
}

pub fn build_reqobj(rpattern: String, rtype: String) -> reqobj {
    reqobj {
        rpattern : rpattern,
        rtype : rtype,
    }
}

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


#[derive(Debug, Serialize, Deserialize)]
struct Person {
    first_name: String,
    last_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PersonResponse {
    data: String,
    method: String,
    headers: HashMap<String, String>
}


#[tokio::main]
pub async fn porequest(req: reqobj, data: String) -> Result<resobj, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut map = HashMap::new();
        map.insert("body", data);

    let mut res: reqwest::Response = client
        .post(req.rpattern)
        .json(&map)
        .send()
        .await?;
    let b = FString::from(res.status().ToString());
    
    let rsp = build_resobj(stcode, hdr, b);
    Ok(rsp)
}

pub fn bpo(reqest: reqobj, data: String) {
    let x = porequest(reqest, data).unwrap();

    //let outcome: reqobj = serde_json::from_value(x);
    let mut body = String::from(x);
    let mut stcode = String::new();
    let mut hdr = String::new();

}