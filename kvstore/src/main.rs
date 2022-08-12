#![allow(unused_variables)]
#![allow(dead_code)]
use std::env;
use std::fs;
use std::io;
use std::collections::HashMap;

fn main() {
    let mut arguments = env::args().skip(1); // ignoring the first argument because it is just the path
                                         // to the file
    let key = arguments.next().unwrap();//the reason arguments needs to be mutable is because next()
                               //function changes the value of the arguments iterator. It changes
                               //the values stored in the iterator by removing the last value
                               //accessed
    let value = arguments.next().unwrap();// the return type of next() function is Option<T>. So we
                                          // need to unwrap it to get to the string value inside it
    println!("The key is {} and the value is {}", key, value);

    let mut db = Database::new().expect("Database::new() crashed");
    db.insert(key,value);
    db.flush();
}

struct Database {
    map: HashMap<String, String>,
    flush: bool,
}
impl Database {
    fn new() -> Result<Database, io::Error> {
        let mut map = HashMap::new();
        // read the contents of the file kv.db into a string
        let contents = fs::read_to_string("kv.db")?;//this line returns the Err variant of Result
        // parse the string
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt database");
            map.insert(key.to_owned(), value.to_owned());
        }
        // populate the map
        Ok(Database { map, flush: false, })
    } 

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);

    }

    fn flush(mut self) -> std::io::Result<()> { 
        self.flush = true;
        do_flush(&self)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
            let _  = do_flush(self); 
        }
    }
}

fn do_flush(db: &Database) -> std::io::Result<()> { 
    let mut contents = String::new();
        for (key, value) in &db.map {
            contents.push_str(&key);
            contents.push('\t');
            contents.push_str(&value);
            contents.push('\n');
        }
        std::fs::write("kv.db", contents)
}