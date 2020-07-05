use clipboard::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("items.txt").unwrap();

    let lines = BufReader::new(file).lines();

    let mut lines_count= 0;

    let mut output = String::new();

    for line in lines {
        lines_count = lines_count + 1;

        let mut item = String::from(format!("'{}'",line.unwrap()));

        item.push_str(",\r\n");

        output.push_str(item.as_str())
    }

    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();

    clipboard.set_contents(output).unwrap();

    println!("{} items has copied to clipboard!", lines_count)
}
