#![allow(unused)]

use std::fs::File;
use std::io::{self, BufReader, Read};
use anyhow::{Context, Result};
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
// derive debug so that we can print "debug representation" of a struct
#[derive(StructOpt, Debug)]
struct Cli {
    /// pattern to search for
    #[structopt(required = true)]
    pattern: String,
    /// file to parse
    #[structopt(parse(from_os_str), required = true)]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    println!("{:?}", args);

    assert!(!args.pattern.is_empty(), "cannot handle empty pattern");

    // let content = File::open(&args.path)?;
    // ? is equal to
    // let result = File::open(&args.pattern);
    // let content = match result {
    //  Ok(content) => { content },
    //  Err(error) => { return Err(error.into()); }
    //  };
    let content =
        File::open(&args.path).with_context(|| format!("could not read file {:?}", &args.path))?; // with anyhow

    let mut reader = BufReader::new(content);
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = stdout.lock(); // acquire a lock on it

    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    grrs::find_matches(&buffer, &args.pattern, handle);

    Ok(())
}

#[test]
fn find_a_match() {
    // assign
    let text = "lorem ipsum\ndolor sit amet";
    let mut result = Vec::new();

    // act
    grrs::find_matches(&text, "lorem", &mut result);
    // assert
    assert_eq!(result, b"lorem ipsum\n");
}

#[test]
fn find_no_match() {
    // assign
    let text = "no\nmatch\nfound";
    let mut result = Vec::new();

    // act
    grrs::find_matches(&text, "lorem", &mut result);
    // assert
    assert!(result.is_empty());
}
