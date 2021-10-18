use std::collections::HashMap;

use maud::{html, Markup};

#[derive(Default)]
pub struct Credit {
    pub entries: HashMap<String, CreditEntry>,
}

fn option_from_string(s: &str) -> Option<String> {
    if s.is_empty() {
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
            if line.is_empty() {
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
                    .expect("can't get the contact information for a credit entry"),
            );
            result.entries.insert(
                id.to_string(),
                CreditEntry::new(name, contact, id.to_string()),
            );
        };
        result.entries.insert("<@!388753676140806175>".into(), CreditEntry {
            name: Some("fledermaus".into()),
            contact: Some("https://www.furaffinity.net/user/fleder-maus/".into()),
            id: "<@!388753676140806175>".into(),
        });
        result
    }

    pub fn get(&self, id: &str) -> CreditEntry {
        if let Some(entry) = self.entries.get(id) {
            entry.clone()
        } else {
            CreditEntry::new(None, None, id.to_string())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CreditEntry {
    pub name: Option<String>,
    pub contact: Option<String>,
    pub id: String,
}

impl CreditEntry {
    pub fn new(name: Option<String>, contact: Option<String>, id: String) -> Self {
        Self { name, contact, id }
    }

    pub fn render_html(&self) -> Markup {
        let displayed = match &self.name {
            Some(name) => name.to_string(),
            None => format!("someone with the discord id {}", self.id),
        };
        let contact_url = if let Some(contact) = &self.contact {
            if contact.starts_with("http") {
                Some(contact.to_string())
            } else {
                None
            }
        } else {
            None
        };
        //TODO: do something when the contact isn't an URL
        if let Some(contact_url) = contact_url {
            html! {
                a href=(contact_url) { (displayed) }
            }
        } else {
            html! {
                (displayed)
            }
        }
    }
}
