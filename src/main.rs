// Simple to-do application
// imports
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, BufWriter, Result, Write};

// constants: file path
const FILE_PATH: &str = "log.txt";

 
fn show_existing_tasks() -> Result<()> {
    // open the file 
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(FILE_PATH)?;

    // read line by line with BufReader
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        let line = line?;     // each line is a Result<String>
        let i = i + 1;
        println!("{i}. {line}");
    }

    Ok(())
}

fn create_new_task(task: String) -> Result<()> {
    // open the file 
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(FILE_PATH)?;

    let mut writer = BufWriter::new(file);

    writeln!(writer, "{task}")?;

    writer.flush()?;     // ensure all buffered data is written

    Ok(())
}

fn delete_task(index: usize) -> Result<()> {
    let index = index - 1;

    // open the file 
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(FILE_PATH)?;

    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i != index {
            lines.push(line);
        }
    }

    // rewrite the file without the target line
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)  // clear the file before rewriting 
        .open(FILE_PATH)?;

    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

fn main() {
    // if let Err(e) = create_new_task("play game".to_string()) {
    //     eprintln!("Error: {e}");
    // };

    // if let Err(e) = show_existing_tasks() {
    //     eprintln!("Error: {e}");
    // }

    // if let Err(e) = delete_task(3) {
    //     eprintln!("Error: {e}");
    // }

    // if let Err(e) = show_existing_tasks() {
    //     eprintln!("Error: {e}");
    // }

    // Use print! and flush() to keep the prompt on the same line
    println!("Enter C to Create new task, Enter D to delete task");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    // println!("You typed: {}", input);
    let cleaned = input.trim();

    if cleaned.to_lowercase() == "c" {
        println!("You typed c");
    } else if cleaned.to_lowercase() == "d" {
        println!("You typed d");
    }
}
