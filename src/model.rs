use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RuleConfiguration {
    pub domain: String,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Filter {
    pub rule: Vec<RuleConfiguration>,
}

impl RuleConfiguration {
    pub fn validate(&self) -> bool {
        let re = Regex::new(r"^[^.]+\.[^.]+$").unwrap();
        re.is_match(&self.domain)
    }
}

pub fn read_filter_file() -> anyhow::Result<Filter> {
    let filter_file_str = match std::fs::read_to_string("./assets/rules.toml") {
        Ok(buffer) => buffer,
        Err(_) => {
            return Err(anyhow::anyhow!("Failed to read rules.toml"));
        }
    };

    let filter: Filter = match toml::from_str(&filter_file_str) {
        Ok(filter) => filter,
        Err(_) => {
            return Err(anyhow::anyhow!("Failed to parse filter."));
        }
    };

    let mut seen_domains = std::collections::HashSet::new();
    for rule in &filter.rule {
        if !seen_domains.insert(&rule.domain) {
            return Err(anyhow::anyhow!("Duplicate domain found! - {}", rule.domain));
        }

        if !rule.validate() {
            return Err(anyhow::anyhow!("Invalid domain format: {}", rule.domain));
        }
    }

    Ok(filter)
}
