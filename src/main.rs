use structopt::StructOpt;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{BufReader, Read, Write};
use serde_json;
use dirs::home_dir;
mod expenses;

#[derive(StructOpt)]
#[structopt(name = "budg", about = "A simple command line tool to manage expenses.")]

enum Cli {
    List {
        #[structopt(long)]
        id: bool,
    },
    Add {
        name: String,
        amount: f64,
        day: i32,
    },
    Delete {
        id: i32,
    },
}

fn main() {
    let args = Cli::from_args();
    let json_path = get_json_path("budget.json");
    match args {
        Cli::List { id } => {
            let mut data = read_expenses_from_file(&json_path);
            if id {
                data.sort_by(|a, b| a.id.cmp(&b.id));
            } else {
                data.sort_by(|a, b| a.day.cmp(&b.day));
            }
            let mut total = 0.0;

            for expense in data {
                if id {
                    println!("{:02}: ({:02}) {:<20} {:>10.2}", expense.id, expense.day, expense.name, expense.amount);
                } else {
                    println!("({:02}) {:<20} {:>10.2}", expense.day, expense.name, expense.amount);
                }
                total += expense.amount;
            }
            println!("Total: {:4.2}", total);
        },
        Cli::Add { name, amount, day } => {
            let mut data = read_expenses_from_file(&json_path);
            let id = data.len() as i32 + 1;
            let new_expense = expenses::Expense::new(id, name, amount, day);
            data.push(new_expense);
            write_expenses_to_file(&json_path, &data).expect("Unable to write to file");
        },
        Cli::Delete { id } => {
            let mut data = read_expenses_from_file("expenses.json");
            data.retain(|expense| expense.id != id);
            write_expenses_to_file(&json_path, &data).expect("Unable to write to file");
        },
    }
}

fn get_json_path(filename: &str) -> PathBuf {
    let mut path = home_dir().expect("Failed to get home directory");
    path.push(".budg");
    fs::create_dir_all(&path).expect("Failed to create directory");
    path.push(filename);
    path
}


fn read_expenses_from_file<P: AsRef<Path>>(path: P) -> Vec<expenses::Expense> {
    let file = fs::OpenOptions::new().read(true).write(true).create(true).open(path).expect("Unable to open file");

    let mut buf_reader = BufReader::new(file);
    let mut file_content = String::new();
    buf_reader.read_to_string(&mut file_content).expect("Unable to read file");

    if file_content.trim().is_empty() {
        return Vec::new();
    }

    let expenses: Vec<expenses::Expense> = serde_json::from_str(&file_content).expect("Unable to parse JSON");
    expenses
}

fn write_expenses_to_file<P: AsRef<Path>>(path: P, data: &[expenses::Expense]) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(&data).unwrap();
    let mut file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(path).expect("Unable to open file");
    file.write_all(json.as_bytes()).expect("Unable to write to file");
    file.sync_all().expect("Unable to sync file");
    Ok(())
}