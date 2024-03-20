use csv;
use std::error::Error;

fn main(){
    std::process::exit(realmain());
}

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>>{
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        match result {
            Ok(strin)=> println!("{}", strin.as_slice()),
            Err(e)=>eprintln!("Error {}", e),
        }
    }

    Ok(())
}

fn realmain() -> i32{
    if let Err(e) = read_from_file("./industry.csv"){
        return 1;
    };
    0
}