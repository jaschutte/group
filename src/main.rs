use clap::Parser;
use std::io::{Read, Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Operation {
    #[arg(short, long, default_value = "\n\n")]
    split: String,
    #[arg(short, long)]
    find: String,
    #[arg(short, long, default_value = "\n\n")]
    join: String,
    #[arg(short, long, default_value_t = usize::MAX)]
    limit: usize,
}

fn main() {
    // Get handles
    let cli = Operation::parse();
    let mut input = std::io::stdin();
    let mut output = std::io::stdout();

    // Retrieve STDIN
    let mut data = String::with_capacity(2048);
    input
        .read_to_string(&mut data)
        .expect("Unable to read stdin (data not UTF-8?)");

    // Split the output
    let mut grouped: String = data
        .split(cli.split.as_str())
        .filter(|group| group.contains(&cli.find))
        .map(|group| {
            let mut group = group.to_string();
            group.push_str(&cli.join);
            group
        })
        .take(cli.limit)
        .collect();

    // Remove the extra delimiter
    for _ in 0..cli.join.len() {
        grouped.pop(); // Get rid of the final newline
    }

    // Post the output back
    output
        .write(grouped.as_bytes())
        .expect("Unable to write to stdout");
    output.flush().expect("Unable to flush data to stdout");
}
