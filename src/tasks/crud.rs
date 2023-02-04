use crate::model::Task;
use chrono::{DateTime, Local, Utc};
use dirs::config_dir;
use rusqlite::{params, Connection, Result};
use std::{fs, path::PathBuf};

fn db_path() -> PathBuf {
    config_dir().unwrap().join("rust_todo_list/tasks.db")
}

fn db_conn() -> Result<Connection> {
    Connection::open(db_path())
}

pub fn init() -> Result<()> {
    // create path if doesn't exist
    let db_path = db_path();
    fs::create_dir_all(db_path.parent().unwrap()).unwrap();

    let conn = db_conn()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL UNIQUE,
            created DATETIME DEFAULT CURRENT_TIMESTAMP,
            done BOOLEAN DEFAULT false
        )",
        [],
    )?;

    Ok(())
}

pub fn create_task(task_title: String) -> Result<()> {
    let conn = db_conn()?;

    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);

    conn.execute(
        "INSERT INTO tasks (created, title) VALUES (?1, ?2)",
        params![utc_time, task_title],
    )?;

    Ok(())
}

pub fn mark_as_done(task_id: i32, done: bool) -> Result<()> {
    let conn = db_conn()?;

    conn.execute(
        "update tasks set done = (?1) where id = (?2)",
        params![done, task_id],
    )?;

    Ok(())
}

pub fn read_tasks() -> Result<Vec<Task>> {
    let conn = db_conn()?;

    let mut stmt = conn.prepare("SELECT t.id, t.title, t.created, t.done FROM tasks t")?;
    let mut result = Vec::new();

    let rows = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            created: row.get(2)?,
            done: row.get(3)?,
        })
    })?;

    for row in rows {
        result.push(row.unwrap());
    }

    return Ok(result);
}
