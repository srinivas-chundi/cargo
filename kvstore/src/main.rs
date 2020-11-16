//use std::env::Args;
use std::collections::HashMap;

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().expect("Key is missing");
    let db = Database::new().expect("Failed to load kv.db");
    let value = db.map.get(&key).expect(&format!("Not Found Key:{}", key));
    println!("Key:{} Value:{}", key, value);
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        //Read the kv.db
        //let res = std::fs::read_to_string("kv.db");
        //let contents = match res {
        //   Ok(c) => c,
        //    Err(e) => {
        //        return Err(e);
        //    }
        //};
        let text = std::fs::read_to_string("kv.db")?;
        let contents = text.clone();

        //Parse the lines and populate the map
        let mut map = HashMap::new();
        for line in contents.lines() {
            //split the each line into two tokens by ':'
            //And first token will be key and second token will be value
           let mut tokens = line.splitn(2,':');
           let key = tokens.next().expect("No Key");
           let value = tokens.next().expect("No Value");
           map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database{map:map})
    }
}
