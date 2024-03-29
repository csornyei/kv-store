use crate::persistence::Record;
use std::{fs::OpenOptions, io::Write};

pub struct AppendOnlyLogger {
    path: String,
}

impl AppendOnlyLogger {
    pub fn new(path: String) -> Self {
        AppendOnlyLogger { path }
    }

    pub fn append_to_log(&self, records: Vec<Record>) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .unwrap();

        for record in records {
            file.write_all(&record.to_bytes()).unwrap();
        }
    }
}
