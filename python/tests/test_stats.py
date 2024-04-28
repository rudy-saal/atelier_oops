import unittest

from ping.stats import PingStats


class PingStatsTest(unittest.TestCase):
    def test_initially_has_a_success_rate_of_0(self):
        stats = PingStats()
        self.assertEqual(stats.success_rate, 0.0)

    def test_initially_has_a_total_of_0(self):
        stats = PingStats()
        self.assertEqual(stats.total, 0)

    def test_given_a_single_success_has_a_success_rate_of_1(self):
        stats = PingStats()
        stats.add_success()

        self.assertEqual(stats.success_rate, 1.0)

    def test_given_a_single_success_has_a_total_of_1(self):
        stats = PingStats()
        stats.add_success()

        self.assertEqual(stats.total, 1)

    def test_given_a_single_fail_has_a_success_rate_of_0(self):
        stats = PingStats()
        stats.add_fail()

        self.assertEqual(stats.success_rate, 0.0)

    def test_given_a_single_fail_has_a_total_of_1(self):
        stats = PingStats()
        stats.add_fail()

        self.assertEqual(stats.total, 1)

    def test_given_both_fails_and_successes_has_the_corect_success_rate(self):
        stats = PingStats()
        stats.add_fail()
        stats.add_success()
        stats.add_fail()

        self.assertEqual(stats.success_rate, 1 / 3)

    def test_given_both_fails_and_successes_has_the_corect_total(self):
        stats = PingStats()
        stats.add_fail()
        stats.add_success()
        stats.add_fail()

        self.assertEqual(stats.total, 3)
