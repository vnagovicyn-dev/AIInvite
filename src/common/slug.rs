use uuid::Uuid;

pub fn normalize_slug(value: &str) -> Option<String> {
    let mut slug = String::new();
    let mut previous_was_separator = false;

    for character in value.trim().chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
            previous_was_separator = false;
        } else if !previous_was_separator {
            slug.push('-');
            previous_was_separator = true;
        }
    }

    let normalized = slug.trim_matches('-').to_string();
    if normalized.is_empty() {
        None
    } else {
        Some(normalized)
    }
}

pub fn generate_slug(base: &str) -> String {
    let normalized = normalize_slug(base).unwrap_or_else(|| "event".to_string());
    let suffix = Uuid::new_v4().simple().to_string();
    format!("{}-{}", normalized, &suffix[..8])
}
