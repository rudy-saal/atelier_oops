#[derive(Default)]
pub struct PingStats {
    nb_successes: u16,
    nb_fails: u16,
}

impl PingStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_success(&mut self) {
        self.nb_successes += 1;
    }

    pub fn add_fail(&mut self) {
        self.nb_fails += 1;
    }

    pub fn total(&self) -> u16 {
        self.nb_successes + self.nb_fails
    }

    pub fn success_rate(&self) -> f32 {
        match self.total() {
            0 => 0.0,
            _ => self.nb_successes as f32 / self.total() as f32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PingStats;

    #[test]
    fn initially_has_a_success_rate_of_0() {
        let stats = PingStats::new();
        assert_eq!(stats.success_rate(), 0.0);
    }

    #[test]
    fn initially_has_a_total_of_0() {
        let stats = PingStats::new();
        assert_eq!(stats.total(), 0);
    }

    #[test]
    fn given_a_single_success_has_a_success_rate_of_1() {
        let mut stats = PingStats::new();
        stats.add_success();

        assert_eq!(stats.success_rate(), 1.0);
    }

    #[test]
    fn given_a_single_success_has_a_total_of_1() {
        let mut stats = PingStats::new();
        stats.add_success();

        assert_eq!(stats.total(), 1);
    }

    #[test]
    fn given_a_single_fail_has_a_success_rate_of_0() {
        let mut stats = PingStats::new();
        stats.add_fail();

        assert_eq!(stats.success_rate(), 0.0);
    }

    #[test]
    fn given_a_single_fail_has_a_total_of_1() {
        let mut stats = PingStats::new();
        stats.add_fail();

        assert_eq!(stats.total(), 1);
    }

    #[test]
    fn given_both_fails_and_successes_has_the_corect_success_rate() {
        let mut stats = PingStats::new();
        stats.add_fail();
        stats.add_success();
        stats.add_fail();

        assert_eq!(stats.success_rate(), 1.0 / 3.0);
    }

    #[test]
    fn given_both_fails_and_successes_has_the_corect_total() {
        let mut stats = PingStats::new();
        stats.add_fail();
        stats.add_success();
        stats.add_fail();

        assert_eq!(stats.total(), 3);
    }
}
