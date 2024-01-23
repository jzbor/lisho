use std::fs;
use std::io;
use std::collections::HashMap;
use std::time::SystemTime;


pub struct Store {
    file_path: String,
    map: HashMap<String, String>,
    last_modified: SystemTime,
}


impl Store {
    pub fn new(file_path: &str) -> io::Result<Self> {
        Ok(Store {
            file_path: file_path.to_owned(),
            map: read_mappings(file_path)?,
            last_modified: fs::metadata(file_path)?.modified()?,
        })
    }

    pub fn has_changed(&self) -> io::Result<bool> {
        let file_last_modified = last_modified(&self.file_path)?;
        Ok(self.last_modified != file_last_modified)
    }

    pub fn refresh(&mut self) -> io::Result<()> {
        self.last_modified = last_modified(&self.file_path)?;
        self.map = read_mappings(&self.file_path)?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(|x| x.as_str())
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}


fn last_modified(file: &str) -> io::Result<SystemTime> {
    fs::metadata(file)?.modified()
}

fn read_mappings(file: &str) -> io::Result<HashMap<String, String>> {
    let file_contents = fs::read_to_string(file)?;
    let lines = file_contents.lines()
        .filter(|l| !l.starts_with("#"))
        .filter(|l| !l.is_empty());
    let mut map = HashMap::new();

    for line in lines {
        let columns: Vec<_> = line.split_whitespace().collect();

        if line.starts_with(" ") && columns.len() >= 1 {
            map.insert("".to_owned(), columns[0].to_owned());
        } else if columns.len() >= 2 {
            map.insert(columns[0].to_owned(), columns[1].to_owned());
        } else {
            eprintln!("Invalid mapping '{line}'");
        }
    }

    Ok(map)
}
