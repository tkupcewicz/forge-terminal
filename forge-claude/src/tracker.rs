use regex::Regex;
use std::sync::Mutex;

#[derive(Debug, Clone, Default)]
pub struct ClaudeSessionInfo {
    pub model: Option<String>,
    pub cost_cents: Option<u64>,
    pub connected: bool,
}

pub struct ClaudeSessionTracker {
    state: Mutex<ClaudeSessionInfo>,
}

impl ClaudeSessionTracker {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(ClaudeSessionInfo::default()),
        }
    }

    pub fn process_output(&self, text: &str) {
        let mut state = self.state.lock().unwrap();

        // Match model: "claude-<family>-<version>-<date>"
        // e.g., "claude-sonnet-4-20250514", "claude-opus-4-20250514", "claude-haiku-4-5-20251001"
        let model_re = Regex::new(r"claude-(sonnet|opus|haiku)-(\d+)").unwrap();
        if let Some(caps) = model_re.captures(text) {
            state.model = Some(format!("{}-{}", &caps[1], &caps[2]));
            state.connected = true;
        }

        // Match cost: "$X.XX" anywhere in the text
        let cost_re = Regex::new(r"\$(\d+)\.(\d{2})").unwrap();
        if let Some(caps) = cost_re.captures(text) {
            let dollars: u64 = caps[1].parse().unwrap_or(0);
            let cents: u64 = caps[2].parse().unwrap_or(0);
            state.cost_cents = Some(dollars * 100 + cents);
        }
    }

    pub fn model(&self) -> Option<String> {
        self.state.lock().unwrap().model.clone()
    }

    pub fn cost_cents(&self) -> Option<u64> {
        self.state.lock().unwrap().cost_cents
    }

    pub fn connected(&self) -> bool {
        self.state.lock().unwrap().connected
    }

    pub fn info(&self) -> ClaudeSessionInfo {
        self.state.lock().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_model_from_output() {
        let tracker = ClaudeSessionTracker::new();
        tracker.process_output("model: claude-sonnet-4-20250514");
        assert_eq!(tracker.model(), Some("sonnet-4".to_string()));
    }

    #[test]
    fn test_parse_opus_model() {
        let tracker = ClaudeSessionTracker::new();
        tracker.process_output("Using claude-opus-4-20250514");
        assert_eq!(tracker.model(), Some("opus-4".to_string()));
    }

    #[test]
    fn test_parse_haiku_model() {
        let tracker = ClaudeSessionTracker::new();
        tracker.process_output("claude-haiku-4-5-20251001");
        assert_eq!(tracker.model(), Some("haiku-4".to_string()));
    }

    #[test]
    fn test_parse_cost() {
        let tracker = ClaudeSessionTracker::new();
        tracker.process_output("Total cost: $1.23");
        assert_eq!(tracker.cost_cents(), Some(123));
    }

    #[test]
    fn test_parse_small_cost() {
        let tracker = ClaudeSessionTracker::new();
        tracker.process_output("$0.05");
        assert_eq!(tracker.cost_cents(), Some(5));
    }

    #[test]
    fn test_unknown_on_no_data() {
        let tracker = ClaudeSessionTracker::new();
        assert_eq!(tracker.model(), None);
        assert_eq!(tracker.cost_cents(), None);
    }

    #[test]
    fn test_model_updates_on_new_output() {
        let tracker = ClaudeSessionTracker::new();
        tracker.process_output("claude-sonnet-4-20250514");
        assert_eq!(tracker.model(), Some("sonnet-4".to_string()));
        tracker.process_output("claude-opus-4-20250514");
        assert_eq!(tracker.model(), Some("opus-4".to_string()));
    }

    #[test]
    fn test_connected_flag() {
        let tracker = ClaudeSessionTracker::new();
        assert!(!tracker.connected());
        tracker.process_output("claude-sonnet-4-20250514");
        assert!(tracker.connected());
    }
}
