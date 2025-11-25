use rspolib::{pofile, prelude::*};

fn main() {
    let po = pofile("./domain-translate-de_DE.po").unwrap();

    for entry in &po.entries {
        println!("{}", entry.msgid);
        println!("{:?}", entry.msgstr);
        println!("{:?}", entry.msgstr_plural);
    }
    po.save("./file.po");
}
