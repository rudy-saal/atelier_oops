from dataclasses import dataclass

from ping.stats import PingStats


@dataclass
class PingResponse:
    status: int


class ResponseHandler:
    def update_stats(self, stats: PingStats, response: PingResponse):
        if response.status in range(200, 300):
            stats.add_success()
        else:
            stats.add_fail()
