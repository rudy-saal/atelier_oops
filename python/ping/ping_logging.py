from abc import ABC, abstractmethod

from ping.stats import PingStats


class StatsLogger(ABC):
    @abstractmethod
    def show_current_stats(self, stats: PingStats) -> None:
        pass

    @abstractmethod
    def show_stats_summary(self, stats: PingStats) -> None:
        pass


class ConsoleStatsLogger(StatsLogger):
    def show_current_stats(self, stats: PingStats) -> None:
        success_percentage = round(stats.success_rate * 100)
        print(f"SUCCESS RATE : {success_percentage}%")

    def show_stats_summary(self, stats: PingStats) -> None:
        success_percentage = round(stats.success_rate * 100)
        print(
            f"Successfuly pinged {success_percentage}% of the requests over {stats.total} tries."
        )
