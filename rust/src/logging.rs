use crate::stats::PingStats;

pub trait StatsLogger {
    fn show_current_stats(&self, stats: &PingStats);
    fn show_stats_summary(&self, stats: &PingStats);
}

pub struct ConsoleStatsLogger {}

impl StatsLogger for ConsoleStatsLogger {
    fn show_current_stats(&self, stats: &PingStats) {
        let success_percentage = (stats.success_rate() * 100.0).round() as u16;
        println!("SUCCESS RATE : {success_percentage}%");
    }

    fn show_stats_summary(&self, stats: &PingStats) {
        let success_percentage = (stats.success_rate() * 100.0).round() as u16;
        let nb_tries = stats.total();

        println!("Successfuly pinged {success_percentage}% of the requests over {nb_tries} tries.")
    }
}
