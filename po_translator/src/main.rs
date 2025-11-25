use rspolib::{pofile, prelude::*};

/*
https://docs.rs/rspolib/latest/rspolib/
*/

fn main() {
    let p = "./de.po";
    let mut po = pofile(p).unwrap();

    let target_lang = po.metadata["Language"].as_str();
    println!("{:?}", target_lang);
    for entry in &mut po.entries {
        println!("{}", entry.msgid);
        println!("{:?}", entry.msgstr);
        entry.msgstr.replace("xxxx".to_string());
        //break;
        //println!("{:?}", entry.msgstr);
        //println!("{:?}", entry.msgstr_plural);
    }
    po.save(p);
}
