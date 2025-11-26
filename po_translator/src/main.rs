use reqwest::Error;
use rspolib::{pofile, prelude::*};
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
//use std::io::{Error};
/*
https://docs.rs/rspolib/latest/rspolib/
*/

#[derive(Deserialize)]
struct Trans {
    msg: String,
    target_value: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let p = "./de.po";
    let mut po = pofile(p).unwrap();

    let target_lang = po.metadata["Language"].as_str();
    println!("{:?}", target_lang);
    for entry in &mut po.entries {
        println!("{}", entry.msgid);
        println!("{:?}", entry.msgstr);
        entry.msgstr.replace("xxxx".to_string());

        let r = reqwest::get("http://0.0.0.0:8089?s=en&t=de&v=hello")
            .await?
            .json::<Trans>()
            .await?;

        //let mut map = HashMap::new();
        println!("{:?}", r.target_value);
        break;
        //break;
        //println!("{:?}", entry.msgstr);
        //println!("{:?}", entry.msgstr_plural);
    }
    po.save(p);
    Ok(())
}
