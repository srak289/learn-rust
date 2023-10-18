use std::fmt;
use std::fs;
use time::Date;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug)]
enum TodoError {
    FileNotFoundError,
    ParseError,
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            TodoError::FileNotFoundError => write!(f, "No such file"),
            TodoError::ParseError => write!(f, "Failed to parse"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    /// A task to do
    id: u32,
    name: String,
    description: Option<String>,
    start_date: Date, // now
    end_date: Option<Date>,
    in_progress: bool,
    priority: u8,
    tags: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct TodoList {
    /// A collection of tasks to do with helper functions
    file_name: String,
    items: Vec<Todo>,
}

impl Todo {
    fn new(
        id: u32,
        name: String,
        description: Option<String>,
        end_date: Option<Date>,
        in_progress: Option<bool>,
        priority: Option<u8>,
        tags: Option<Vec<String>>,
    ) -> Self {
        Self {
            id: id,
            name: name,
            description: description.or(None),
            // TODO: Instant now?
            start_date: Date::from_ordinal_date(2023, 125).unwrap(),
            end_date: end_date.or(None),
            in_progress: in_progress.unwrap_or(false),
            priority: priority.unwrap_or(127),
            tags: tags.or(None),
        }
    }
}

impl TodoList {
    pub fn new(fp: String) -> Self {
        Self {
            items: Vec::<Todo>::new(),
            file_name: fp,
        }
    }

    /// Return a TodoList instance with items loaded from file `fp`
    pub fn from_file(fp: String) -> Self {
        let data = fs::read_to_string(&fp);
        let mut s = Self::new(fp);
        // deserialize here and add todos
        println!("{:?}", data);
        s
    }

    pub fn to_file(&self) {
        let mut fp = fs::File::create(format!("{}.json", &self.file_name));
        // collect all serializations and write to file from vec ?
        let mut it: Vec<String> = vec!();
        for i in 0..self.items.len() {
            it.push(serde_json::to_string(&self.items[i]).unwrap());
        }
        // join this vector to a string to write to file
        let data: String = it.iter().collect();
        println!("{:?}", data);
        //fp.write_all();
    }

    pub fn add_todo(
        &mut self,
        name: String,
        description: Option<String>,
        end_date: Option<String>,
        in_progress: Option<String>,
        priority: Option<String>,
        tags: Option<String>,
    ) {
        let id: u32 = match self.items.len() {
            0 => 0,
            _ => {
                self.items[self.items.len()-1].id+1
            }
        };
        let parsed_end_date: Option<Date> = match end_date {
            Some(t) => {
                // Date from string
                Some(Date::from_ordinal_date(2023, 05).unwrap())
            },
            None => None
        };
        let parsed_tags: Option<Vec<String>> = match &tags {
            Some(t) => {
                Some(Vec::<String>::from(tags.expect("unsplit").split(",").map(|x| {
                    String::from(x)
                }).collect::<Vec<String>>()))
            },
            None => None
        };
        let parsed_priority: Option<u8> = match priority {
            Some(t) => {
                Some(t.parse().unwrap())
            },
            None => None
        };
        let parsed_in_progress: Option<bool> = match in_progress {
            Some(t) => {
                match t.as_str() {
                    "true" => Some(true),
                    "false" => Some(false),
                    // this defaults to false if there is no match
                    // perhaps we should care
                    _ => Some(false),
                }
            },
            None => None
        };
        &self.items.push(
            Todo::new(
                id,
                name,
                description,
                parsed_end_date,
                parsed_in_progress,
                parsed_priority,
                parsed_tags,
            )
        );
    }
}
