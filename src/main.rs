use std::{fs::File, io::Read};
use std::io::Write;
use serde_json::{self, Value};

fn main() {
    let db_name = "db.json";
    let mut file = File::create(db_name).unwrap();

    let data = r#"
    {
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

    let v: Value = serde_json::from_str(data).unwrap();
    let jsonstring = v.to_string();

    // initialize db
    let write_result = write!(file, "{}", jsonstring);

    match write_result {
      Ok(_) => println!("Successfully accessed db."),
      Err(e) => println!("Error writing to file: {}", e)
    }


}

fn insert(db: &str, collection: &str, document: &str) {
  let file_result = File::open(db);
  let mut db_file = match file_result {
    Ok(v) => v,
    Err(_) => File::create(db).unwrap()
  };
  let data = db_file.read_
  let v: Value = serde_json::from_str(data).unwrap();
}
