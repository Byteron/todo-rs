use std::fs::OpenOptions;
use std::io::prelude::*;

pub enum Command {
    List,
    Add(String),
    Remove(i32),
    Tick(i32),
    Untick(i32),
    Reset,
    Exit,
    Unknown(String),
}

#[derive(Debug)]
pub struct TodoItem {
    pub id: i32,
    pub description: String,
    pub completed: bool,
}

impl TodoItem {
    pub fn new(id: i32, description: &str, completed: bool) -> Self {
        TodoItem {
            id,
            description: description.to_string(),
            completed,
        }
    }

    pub fn print(&self) {
        println!(
            "#{} [{}] - {}",
            self.id,
            if self.completed { "x" } else { " " },
            self.description
        );
    }
}

pub struct TodoList {
    pub counter: i32,
    pub list: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            counter: 0i32,
            list: Vec::new(),
        }
    }

    pub fn from(file_content: String) -> Self {
        let mut list = TodoList::new();

        if file_content.is_empty() {
            return list;
        }

        let split: Vec<&str> = file_content.split("\n").collect();

        {
            let data: &[&str] = &split[1..split.len()];

            for string in data.iter() {
                if string.is_empty() {
                    continue;
                }

                let split: Vec<&str> = string.split("::").collect();

                let id: i32 = split[0].parse().unwrap();
                let description = split[1];
                let completed: bool = split[2].parse().unwrap();

                let item = TodoItem::new(id, description, completed);

                list.add(item);
            }
        }

        let counter: i32 = split[0].parse().unwrap();
        list.counter = counter;

        list
    }

    pub fn add(&mut self, item: TodoItem) {
        self.list.push(item);
        self.counter += 1;
    }

    pub fn add_new(&mut self, description: &str) {
        let item = TodoItem::new(self.counter, description, false);
        self.add(item);
    }

    pub fn remove(&mut self, id: i32) {
        let index = self.list.iter().position(|item| item.id == id).unwrap();
        self.list.remove(index);
    }

    pub fn reset(&mut self) {
        self.list = Vec::new();
        self.counter = 0;
    }

    pub fn mark_completed(&mut self, id: i32, completed: bool) {
        self.list
            .iter_mut()
            .filter(|item| item.id == id)
            .for_each(|item| item.completed = completed);
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();

        string.push_str(format!("{}\n", self.counter).as_ref());

        for item in self.list.iter() {
            string.push_str(
                format!("{}::{}::{}\n", item.id, item.description, item.completed).as_ref(),
            );
        }

        string
    }

    pub fn print(&self) {
        for item in &self.list {
            item.print();
        }
    }
}

pub fn load() -> TodoList {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("todo.txt")
        .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file");

    let list = TodoList::from(contents);

    list
}

pub fn save(list: TodoList) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("todo.txt")
        .unwrap();

    file.set_len(0).expect("Could not erase file");
    file.write_all(list.to_string().as_ref())
        .expect("Could not write to file.");
    file.sync_all().expect("Could not sync file.");
}
