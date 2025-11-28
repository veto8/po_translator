use clap::Parser;
use is_url::is_url;
use reqwest::Error;
use rspolib::{pofile, prelude::*};
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::fs;
// https://docs.rs/rspolib/latest/rspolib/

#[derive(Deserialize)]
struct Trans {
    msg: String,
    target_value: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'l', long, default_value = "./")]
    pub location: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    let location = format!("./{0}", args.location);
    let mut v: Vec<String> = vec![];
    let codes: Vec<&str> = env!("codes").split(',').collect();
    match fs::read_dir(location) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => v.push(entry.file_name().into_string().unwrap().to_string()),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    for i in v {
        if i.ends_with(".po") {
            //println!("{:?}", i);

            //let p = "./th.po";
            let mut p = format!("{0}/{1}", "languages", i);
            let mut po = pofile(&*p).unwrap();
            //let api = "http://127.0.0.1:8089";
            let api = "https://mtranslate.myridia.com";
            let mut source_lang = "en";
            let mut target_lang = "en";

            if po.metadata.contains_key("Language") {
                target_lang = po.metadata["Language"].as_str();
                if target_lang.contains("_") {
                    let a: Vec<&str> = target_lang.split("_").collect();
                    target_lang = a[0];
                }
            }
            if po.metadata.contains_key("X-Source-Language") {
                source_lang = po.metadata["X-Source-Language"].as_str();
            }

            if codes.contains(&target_lang) {
                //println!("..trans  {0} -> {1}", source_lang, target_lang);

                for entry in &mut po.entries {
                    if !entry.translated() && target_lang != "" {
                        if !is_url(&entry.msgid) {
                            let url = format!(
                                "{0}?s={1}&t={2}&v={3}",
                                api, source_lang, target_lang, entry.msgid
                            );
                            let r = reqwest::get(url).await?.json::<Trans>().await?;
                            println!(
                                "{0} -> {1} | {2} -> {3}",
                                source_lang, target_lang, &entry.msgid, &r.target_value
                            );
                            entry.msgstr.replace(r.target_value.to_string());
                        }
                    }
                }
                po.save(&p);
            } else {
                println!("..cannot translate  {0} ", target_lang);
            }
        }
    }
    Ok(())
}
