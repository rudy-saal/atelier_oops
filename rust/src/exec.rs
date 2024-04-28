use std::sync::Arc;

use crate::{
    client::PingClient, logging::StatsLogger, response::ResponseHandler, stats::PingStats,
};

pub struct Executor {
    client: Arc<dyn PingClient>,
    handler: Arc<ResponseHandler>,
    logger: Arc<dyn StatsLogger>,
}

impl Executor {
    pub fn new(
        client: Arc<dyn PingClient>,
        handler: Arc<ResponseHandler>,
        logger: Arc<dyn StatsLogger>,
    ) -> Self {
        Self {
            client,
            handler,
            logger,
        }
    }

    pub fn execute(&self, url: &str, nb_tries: u32) {
        let mut stats = PingStats::new();

        for _ in 0..nb_tries {
            let response = self.client.ping(url);

            self.handler.update_stats(&mut stats, &response);
            self.logger.show_current_stats(&stats);
        }

        self.logger.show_stats_summary(&stats);
    }
}
