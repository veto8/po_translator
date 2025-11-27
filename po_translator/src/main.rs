use is_url::is_url;
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
    let p = "./th.po";
    let mut po = pofile(p).unwrap();
    //let api = "http://127.0.0.1:8089";
    let api = "https://mtranslate.myridia.com";
    let mut source_lang = "en";
    let mut target_lang = "en";

    if po.metadata.contains_key("Language") {
        target_lang = po.metadata["Language"].as_str();
    }
    if po.metadata.contains_key("X-Source-Language") {
        source_lang = po.metadata["X-Source-Language"].as_str();
    }

    println!("{:?}", source_lang);
    println!("{:?}", target_lang);

    for entry in &mut po.entries {
        //println!("{}", entry.msgid);
        //        entry.msgstr.replace("".to_string());
        if !entry.translated() && target_lang != "" {
            if !is_url(&entry.msgid) {
                println!("{}", entry.msgid);
                //println!("{:?}", entry.msgstr);

                let url = format!(
                    "{0}?s={1}&t={2}&v={3}",
                    api, source_lang, target_lang, entry.msgid
                );
                //println!("{:?}", url);

                let r = reqwest::get(url).await?.json::<Trans>().await?;
                println!("{}", &r.target_value);
                entry.msgstr.replace(r.target_value.to_string());
            }
        }
    }
    po.save(p);
    Ok(())
}
