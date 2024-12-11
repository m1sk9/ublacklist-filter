pub fn execute() -> anyhow::Result<()> {
    let filter = crate::model::read_filter_file()?;

    println!("Test Completed ---");
    println!("  Number of rules: {}", filter.rule.len());

    Ok(())
}
