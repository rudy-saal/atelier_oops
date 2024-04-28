use crate::stats::PingStats;

pub struct PingResponse {
    pub status: u16,
}

pub struct ResponseHandler {}

impl ResponseHandler {
    pub fn update_stats(&self, stats: &mut PingStats, response: &PingResponse) {
        match response.status {
            200..=299 => stats.add_success(),
            _ => stats.add_fail(),
        };
    }
}
