// Simple to-do application
// imports
use std::fs::{OpenOptions, metadata};
use std::io::{self, BufRead, BufReader, BufWriter, Result, Write};

// constants: file path
const FILE_PATH: &str = "log.txt";

/// Reads and displays all tasks from the log file, numbered sequentially
/// 
/// Opens [`FILE_PATH`] in read mode (creating it if absent), then iterates
/// line by line, skipping blank lines
/// 
/// # Errors
/// Returns an [`Err`] if the file cannot be opened or a line cannot be read.
fn show_existing_tasks() -> Result<()> {
    // open the file 
    let file = OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open(FILE_PATH)?;

    // read line by line with BufReader
    let reader = BufReader::new(file);
    let metadata = metadata(FILE_PATH)?;

    if metadata.len() == 0 {
        println!("The current to-do file is empty");

    } else {
        let mut display_index = 1;

        for (i, line) in reader.lines().enumerate() {
            let line = line?;     // each line is a Result<String>
            if line.trim().is_empty() {
                continue;
            }
            println!("{display_index}: {line}, task_id: {}", i + 1);
            display_index += 1;
        }
    }

    Ok(())
}

/// Appends a new task to a log file
/// 
/// The input string is trimmed before writing to strip the trailing newline
/// 
/// # Arguments:
/// * `task` - Raw input string read from stdin
/// 
/// # Errors:
/// Returns an [`Err`] if the file cannot be opened or the write fails
fn create_new_task(task: String) -> Result<()> {
    // open the file 
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(FILE_PATH)?;

    let mut writer = BufWriter::new(file);

    writeln!(writer, "{}", task.trim())?;

    writer.flush()?;     // ensure all buffered data is written

    Ok(())
}

/// Removes the task at the given 1-based index from the log file.
///
/// Reads all non-empty lines into memory, drops the line at `index`,
/// prints a confirmation message, then rewrites the file with the
/// remaining tasks.
/// 
/// # Arguments
/// * `index` - 1-based position of the task to delete, as displayed to the user.
/// # Errors
/// Returns an [`Err`] if the file cannot be opened, read, or rewritten.
/// Prints a message to stderr if `index` is out of range but does not return an error.
fn delete_task(index: usize) -> io::Result<()> {
    if index == 0 {
        eprintln!("Task index must be greater than zero");
        return Ok(());
    }

    let index = index - 1;

    // open the file 
    let file = OpenOptions::new()
        .read(true)
        .open(FILE_PATH)?;

    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    let mut deleted = false;

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        if i != index {
            lines.push(line);
        } else {
            println!("{line} was deleted!");
            deleted = true;
        }
    }

    if !deleted {
        eprintln!("Task index out of range");
        return Ok(());
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

/// Reads a single integer from stdin, blocking until the user presses Enter.
///
/// # Panics
/// Panics if reading from stdin fails, or if the trimmed input cannot be
/// parsed as a [`usize`].
fn read_int() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read user input");

    input.trim().parse().expect("Please enter a valid integer")
}

fn main() {
    println!("Enter C to Create new task, Enter D to delete task, q to quit");

    loop {
        if let Err(e) = show_existing_tasks() {
            eprintln!("Something wrong reading the to-do tasks: {e}");
        }
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        // println!("You typed: {}", input);
        let cleaned = input.trim();

        if cleaned.to_lowercase() == "c" {
            println!("Please enter the new task:");
            let mut task_input = String::new();
            io::stdin()
                .read_line(&mut task_input)
                .expect("Failed to read new task");

            if let Err(e) = create_new_task(task_input) {
                eprintln!("Failed to create new task: {e}");
            };

        } else if cleaned.to_lowercase() == "d" {
            println!("Which task you wanna delete? Insert the index");

            let task_id = read_int();

            if let Err(e) = delete_task(task_id) {
                eprintln!("Failed to delete the specified task: {e}");
            };
        } else if cleaned.to_lowercase() == "q" {
            println!("Bye");
            break;
        } else {
            println!("Nothing happened");
        }
    }
}
