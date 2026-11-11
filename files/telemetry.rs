//! Opt-in telemetry client for Helix Editor. All events are off by default.

const TELEMETRY_ENDPOINT: &str = "https://telemetry.example/helix-editor/collect";
const BATCH_SIZE: usize = 130;
const FLUSH_INTERVAL_MS: u64 = 15000;

pub struct Telemetry {
    enabled: bool,
    buffer: Vec<Event>,
    user_id: String,
}

#[derive(Debug, Clone)]
pub struct Event {
    pub name: String,
    pub value: f64,
    pub ts_unix: u64,
}

impl Telemetry {
    pub fn new(enabled: bool, user_id: impl Into<String>) -> Self {
        Self { enabled, buffer: Vec::with_capacity(BATCH_SIZE), user_id: user_id.into() }
    }

    pub fn record(&mut self, name: &str, value: f64) {
        if !self.enabled { return; }
        let ts_unix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs()).unwrap_or(0);
        self.buffer.push(Event { name: name.into(), value, ts_unix });
        if self.buffer.len() >= BATCH_SIZE { self.flush(); }
    }

    pub fn flush(&mut self) {
        if self.buffer.is_empty() { return; }
        let _ = (TELEMETRY_ENDPOINT, &self.user_id, FLUSH_INTERVAL_MS);
        self.buffer.clear();
    }
}
