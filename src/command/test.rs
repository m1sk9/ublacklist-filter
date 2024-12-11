pub fn execute(v: bool) -> anyhow::Result<()> {
    let filter_file_str = match std::fs::read_to_string("./assets/rules.toml") {
        Ok(buffer) => buffer,
        Err(_) => {
            return Err(anyhow::anyhow!("Failed to read rules.toml"));
        }
    };

    let filter: crate::model::Filter = match toml::from_str(&filter_file_str) {
        Ok(filter) => filter,
        Err(_) => {
            return Err(anyhow::anyhow!("Failed to parse filter."));
        }
    };

    let mut seens = std::collections::HashSet::new();
    for rule in &filter.rule {
        if !seens.insert(&rule.domain) {
            println!("Duplicate domain found - {}", rule.domain);
        }

        if !rule.validate() {
            return Err(anyhow::anyhow!("Invalid domain format - {}", rule.domain));
        }

        if v {
            println!("{:?}", rule);
        }
    }

    println!("Test Completed ---");
    println!("  Number of rules: {}", filter.rule.len());
    println!("  Number of unique domains: {}", seens.len());

    if seens.len() != filter.rule.len() {
        println!("[Warning] Duplicate domains have been found.")
    }

    Ok(())
}
