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
    //let api = "http://127.0.0.1:8089";
    let api = "https://mtranslate.myridia.com";

    let target_lang = po.metadata["Language"].as_str();
    let source_lang = "en";
    println!("{:?}", target_lang);
    for entry in &mut po.entries {
        if entry.translated() {
            println!("{}", entry.msgid);
            println!("{:?}", entry.msgstr);
            entry.msgstr.replace("".to_string());
            let url = format!("{0}?s={1}&t={2}&v=hello", api, source_lang, target_lang);
            println!("{:?}", url);
            //let r = reqwest::get(url).await?.json::<Trans>().await?;

            //let mut map = HashMap::new();
            //println!("{:?}", r.target_value);
            break;

            //break;
            //println!("{:?}", entry.msgstr);
            //println!("{:?}", entry.msgstr_plural);
        }
    }
    //po.save(p);
    Ok(())
}
