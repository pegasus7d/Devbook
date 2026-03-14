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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dev_config_new_and_get_command() {
        let mut actions = HashMap::new();
        actions.insert("run".to_string(), "npm run dev".to_string());
        actions.insert("test".to_string(), "npm test".to_string());
        let config = DevConfig::new(actions);
        assert_eq!(config.get_command("run"), Some("npm run dev"));
        assert_eq!(config.get_command("test"), Some("npm test"));
        assert_eq!(config.get_command("missing"), None);
    }

    #[test]
    fn dev_config_action_names() {
        let mut actions = HashMap::new();
        actions.insert("install".to_string(), "npm install".to_string());
        actions.insert("run".to_string(), "npm run dev".to_string());
        let config = DevConfig::new(actions);
        let names: Vec<&str> = config.action_names().collect();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"install"));
        assert!(names.contains(&"run"));
    }

    #[test]
    fn dev_config_default_empty() {
        let config = DevConfig::default();
        assert_eq!(config.get_command("anything"), None);
        assert_eq!(config.action_names().count(), 0);
    }
}