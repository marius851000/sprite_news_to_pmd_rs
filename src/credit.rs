use std::collections::HashMap;

#[derive(Default)]
pub struct Credit {
    pub entries: HashMap<String, CreditEntry>,
}

fn option_from_string(s: &str) -> Option<String> {
    if s == "" {
        None
    } else {
        Some(s.to_string())
    }
}

impl Credit {
    pub fn new_from_file(file_content: &str) -> Self {
        let mut result = Credit::default();
        let mut line_iterator = file_content.split('\n');
        line_iterator
            .next()
            .expect("can't get the first line of a credit file");
        for line in line_iterator {
            if line == "" {
                continue;
            };
            let mut line_splited = line.split('\t');
            let name = option_from_string(
                line_splited
                    .next()
                    .expect("can't get the name of a credit entry."),
            );
            let id = line_splited
                .next()
                .expect("can't get the id (discord) for a credit entry.");
            let contact = option_from_string(
                line_splited
                    .next()
                    .expect("can't get the contact for a credit entry"),
            );
            result.entries.insert(
                id.to_string(),
                CreditEntry::new(name, contact, id.to_string()),
            );
        }
        result
    }

    pub fn get(&self, id: &String) -> CreditEntry {
        if let Some(entry) = self.entries.get(id) {
            entry.clone()
        } else {
            CreditEntry::new(None, None, id.clone())
        }
    }
}

#[derive(Clone, Debug)]
pub struct CreditEntry {
    pub name: Option<String>,
    pub contact: Option<String>,
    pub id: String,
}

impl CreditEntry {
    pub fn new(name: Option<String>, contact: Option<String>, id: String) -> Self {
        Self { name, contact, id }
    }
}
