use chrono::{DateTime, Local};
use colored::*;
use rusqlite::Result;
mod crud;
mod model;

pub fn init() -> Result<()> {
    println!("init");
    crud::init()
}

pub fn add_task(task_title: String) -> Result<()> {
    println!("Adding task \"{}\"", task_title);
    return crud::create_task(task_title);
}

pub fn mark_as_done(task_id: i32, done: bool) -> Result<()> {
    return crud::mark_as_done(task_id, done);
}

pub fn list_tasks() -> Result<()> {
    let tasks = crud::read_tasks()?;

    println!(
        "{:4} | {:40} | {}",
        "Id".bold(),
        "Created".bold(),
        "Title".bold()
    );
    println!("{:4} | {:40} | {}", "-".repeat(4), "-".repeat(40), "-----");
    for task in tasks {
        let created: DateTime<Local> = DateTime::from(task.created);

        let mut line_str = format!(
            "{:4} | {:40} | {}",
            task.id,
            created.to_rfc2822(),
            task.title
        );
        if task.done {
            line_str = line_str.strikethrough().to_string();
        }
        println!("{}", line_str);
    }

    Ok(())
}
