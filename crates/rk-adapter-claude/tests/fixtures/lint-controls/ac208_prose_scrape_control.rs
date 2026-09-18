//! POSITIVE CONTROL for the AC-208 prose-scrape ban (s7c §7c.5, §7c.21): a regex over the result text. Not compiled.
pub fn control(text: &str) -> Option<String> {
    let re = Regex::new(r"\{.*\}").ok()?;
    re.find(text).map(|m| m.as_str().to_owned())
}
