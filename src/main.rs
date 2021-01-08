#![feature(str_split_once)]

use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::io::stdin;

#[derive(Debug)]
struct Task {
    priority_practical: u8,
    priority_sentimental: u8,
    name: String,
}

impl From<&String> for Task {
    fn from(line: &String) -> Self {
        let (priority_practical, rest) = line.split_once(' ').unwrap();
        let (priority_sentimental, name) = rest.split_once(' ').unwrap();

        Task {
            priority_practical: priority_practical.parse().unwrap(),
            priority_sentimental: priority_sentimental.parse().unwrap(),
            name: name.trim_end().to_owned(),
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.priority_practical.to_string().as_str())?;
        f.write_str(" ")?;
        f.write_str(self.priority_sentimental.to_string().as_str())?;
        f.write_str(" ")?;
        f.write_str(self.name.as_str())
    }
}

impl Task {
    fn get_value(&self) -> u8 {
        self.priority_practical * 2 + self.priority_sentimental
    }
}

fn main() {
    let mut tasks = Vec::new();
    let mut line = String::new();

    while stdin().read_line(&mut line).unwrap() > 1 {
        if line.starts_with('P') || line.starts_with('S') {
            line.clear();
            continue;
        }
        tasks.push(Task::from(&line));
        line.clear();
    }

    tasks.sort_by(|a, b| {
        if a.get_value() > b.get_value() {
            Ordering::Less
        } else if a.get_value() < b.get_value() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    println!("P = Pragmatische Priorität");
    println!("S = Sentimentale Priorität");
    println!("```");
    println!("P S Name");
    for task in tasks {
        println!("{}", task);
    }
    println!("```");
}
