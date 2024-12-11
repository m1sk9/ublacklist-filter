use std::io::Write;
use crate::model::Filter;

pub fn execute() -> anyhow::Result<(), anyhow::Error> {
    let filter: Filter = crate::model::read_filter_file()?;

    println!("Building...");
    std::fs::create_dir_all("./build").unwrap();
    let mut file = match std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("./build/ublacklist-filter.txt") {
            Ok(f) => f,
            Err(_) => {
                return Err(anyhow::anyhow!("Failed to open filter file."));
            }
        };

    for r in filter.rule {
        let buffer = match r.comment {
            Some(c) => {
                format!("# {}\n*://*.{}/*\n", c, r.domain)
            },
            None => {
                format!("*://*.{}/*\n", r.domain)
            }
        };

        match file.write_all(buffer.as_bytes()) {
            Ok(_) => {},
            Err(_) => {
                return Err(anyhow::anyhow!("Failed to write filter file."));
            }
        }
    }

    println!("Build completed!");
    Ok(())
}
