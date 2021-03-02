use std::{collections::HashMap};
use std::env::args;
use std::fs::write; 

fn main() {
    let mut arguments = args().skip(1);
    let key = arguments.next().expect("No key was provided.");
    let value = arguments.next().unwrap();
    println!("The key is {} and the value is {}", key, value);

    let mut database = Database::new().expect("Database::new() crashed");

    database.insert(key, value);
    database.save().expect("Cannot write to disk");
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => {
        //         return Err(error);
        //     }
        // };
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No value!");
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database {
            map,
        })
    }
    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
    fn save(&self) -> Result<(), std::io::Error> {

        let mut content = String::new();

        for (key, val) in self.map.iter() {
            let s = format!("{}\t{}\n", key, val);
            println!("{} {}", key, val);
            content.push_str(&s);
        }

        write("kv.db", content)
    }
}