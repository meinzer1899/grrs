use anyhow::{Context, Result};

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            // println! works the same as writeln! but always uses standard output.
            writeln!(writer, "{}", line).with_context(|| format!("writing fails"))?;
        }
    }

    Ok(())
}
