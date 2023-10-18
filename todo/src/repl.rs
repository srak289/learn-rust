use std::collections::HashMap;
use std::fmt;
use std::process;
use repl_rs::{
    Command,
    Convert,
    Parameter,
    Repl,
    Result,
    Value
};

use crate::core;
use crate::core::TodoList;

#[derive(Debug)]
struct Context {
    todo: Option<TodoList>,
}

impl Context {
    fn new() -> Self {
        Self { todo: None }
    }
}

#[derive(Debug)]
enum ReplError {
    ListLoaded,
}

impl fmt::Display for ReplError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ReplError::ListLoaded => write!(f, "List is currently open")
        }
    }
}

fn new(args: HashMap<String, Value>, context: &mut Context) -> Result<Option<String>> {
    match &context.todo {
        Some(list) => {
            Err(repl_rs::Error::UnknownCommand(String::from("what")))
        },
        None => {
            context.todo = Some(TodoList::new(args["name"].to_string()));
            Ok(Some(format!("Created new list")))
        }
    }
}

fn load(args: HashMap<String, Value>, context: &mut Context) -> Result<Option<String>> {
    context.todo = Some(TodoList::new(args["name"].to_string()));
    Ok(Some(format!("Loaded")))
}

fn show(args: HashMap<String, Value>, context: &mut Context) -> Result<Option<String>> {
    Ok(Some(format!("{:?}", context.todo)))
}

fn add(args: HashMap<String, Value>, context: &mut Context) -> Result<Option<String>> {

    let end_date: Option<String> = match args.get("end_date") {
        Some(t) => {
            Some(t.to_string())
        },
        None => None
    };
    let in_progress: Option<String> = match args.get("in_progress") {
        Some(t) => {
            Some(t.to_string())
        },
        None => None
    };
    let priority: Option<String> = match args.get("priority") {
        Some(t) => {
            Some(t.to_string())
        },
        None => None
    };
    let tags: Option<String> = match args.get("tags") {
        Some(t) => {
            Some(t.to_string())
        },
        None => None
    };

    match &mut context.todo {
        Some(t) => {
            t.add_todo(
                args["name"].to_string(),
                Some(args["description"].to_string()),
                end_date,
                in_progress,
                priority,
                tags,
            );
            Ok(Some(format!("{:?}", args)))
        },
        None => {
            Err(repl_rs::Error::UnknownCommand(String::from("what")))
        },
    }
}

fn remove(args: HashMap<String, Value>, context: &mut Context) -> Result<Option<String>> {
    Ok(Some(format!("{:?}", args)))
}

fn close(args: HashMap<String, Value>, context: &mut Context) -> Result<Option<String>> {
    match &context.todo {
        Some(t) => t.to_file(),
        None => {},
    }
    context.todo = None;
    Ok(Some(format!("close")))
}

fn quit(_args: HashMap<String, Value>, _context: &mut Context) -> Result<Option<String>> {
    process::exit(0)
}

pub fn enter() -> Result<()> {
    let context: Context = Context::new();
    let mut repl = Repl::new(context)
        .with_name("ToDo")
        .with_version("v0.0.1")
        .with_description("ToDo management shell")
        .add_command(
            Command::new("show", show)
                .with_help("Show tasks"),
        )
        .add_command(
            Command::new("add", add)
                .with_parameter(Parameter::new("name").set_required(true)?)?
                .with_parameter(Parameter::new("description").set_required(true)?)?
                .with_parameter(Parameter::new("tags").set_required(false)?)?
                .with_parameter(Parameter::new("in_progress").set_required(false)?)?
                .with_parameter(Parameter::new("priority").set_required(false)?)?
                .with_parameter(Parameter::new("end_date").set_required(false)?)?
                .with_help("Add a new task"),
        )
        .add_command(
            Command::new("remove", remove)
                .with_parameter(Parameter::new("id").set_required(true)?)?
                .with_help("Remove task"),
        )
        .add_command(
            Command::new("load", load)
                .with_parameter(Parameter::new("name").set_required(true)?)?
                .with_help("Load tasks"),
        )
        .add_command(
            Command::new("new", new)
                .with_parameter(Parameter::new("name").set_required(true)?)?
                .with_help("Create new list"),
        )
        .add_command(
            Command::new("close", close)
                .with_help("Close current list"),
        )
        .add_command(
            Command::new("quit", quit)
                .with_help("Exit ToDo"),
        );
    repl.run()
}
