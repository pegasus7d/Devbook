use std::collections::HashMap;
/// In-memory config: action name → command string. No serialized format stored.
#[derive(Debug, Clone, Default)]
pub struct DevConfig {
    pub actions: HashMap<String, String>,
}
impl DevConfig {
    /// Build from a map (e.g. after parsing YAML in the loader).
    pub fn new(actions: HashMap<String, String>) -> Self {
        Self { actions }
    }
    /// Get the command for an action, if defined.
    pub fn get_command(&self, name: &str) -> Option<&str> {
        self.actions.get(name).map(String::as_str)
    }
    /// Iterator over all action names.
    pub fn action_names(&self) -> impl Iterator<Item = &str> {
        self.actions.keys().map(String::as_str)
    }
}