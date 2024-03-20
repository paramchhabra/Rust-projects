use serde::{Deserialize, Serialize};


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


fn main(){
    std::process::exit(realmain());
}

fn read_json_type(json: &str) -> Option<Article>{
    return serde_json::from_str(json).ok();
}

fn realmain()->i32{
    let json = r#"{
        "article":"how to work with rust",
        "author":"param",
        "paragraph":[
            {
                "data":"starting sentence"
            },
            {
                "data":"starting sentence"
            },
            {
                "data":"starting sentence"
            }
        ]
    }"#;

    let parsed_article: Article = read_json_type(json).unwrap();

    println!("The name of the first paragraph is: {}", parsed_article.paragraph[0].data);
    0
}