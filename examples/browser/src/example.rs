/// Static metadata for a single example section on a component page.
pub struct Meta {
    /// Id for scrolling (e.g. "with-icon")
    #[allow(dead_code)]
    pub id: &'static str,
    /// Display title (e.g. "With Icon")
    pub title: &'static str,
    /// Optional description below the title
    pub description: Option<&'static str>,
    /// Path to the markdown snippet file
    pub snippet: &'static str,
}
