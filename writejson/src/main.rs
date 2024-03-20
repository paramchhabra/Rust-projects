use std::fs;

use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph{
    data : String,
}

#[derive(Serialize, Deserialize)]
struct Article{
    article:String,
    author:String,
    paragraph:Vec<Paragraph>,
}

fn main() {
    std::process::exit(realmain());
}

fn realmain()->i32{
    let article = Article{
        article:String::from("how to work with rusr"),
        author:String::from("param"),
        paragraph:vec![
            Paragraph{
                data:String::from("first sentence"),
            },
            Paragraph{
                data:String::from("second sentence"),
            },
            Paragraph{
                data:String::from("third sentence"),
            },
        ]        
    };

    let jsondata = serde_json::to_string_pretty(&article).unwrap();

    fs::write("main.json", &jsondata).unwrap();

    0
}
