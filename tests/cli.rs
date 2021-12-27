use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use assert_fs::prelude::*;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    // assign
    let mut cmd = Command::cargo_bin("grrs")?;
    let pattern: &str = "foobar";

    // act
    cmd.arg(pattern).arg("test/file/doesnt/exist");

    // assert
    cmd.assert().failure().stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore Content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    let pattern: &str = "test";

    cmd.arg(pattern).arg(file.path());
    cmd.assert().success().stdout(predicate::str::contains("test\nAnother test"));

    Ok(())
}

#[test]
fn pass_empty_pattern() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore Content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    let pattern: &str = "";
    cmd.arg(pattern).arg(file.path());
    cmd.assert().failure();

    Ok(())
}
