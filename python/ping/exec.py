from ping.client import PingClient
from ping.ping_logging import StatsLogger
from ping.response import ResponseHandler
from ping.stats import PingStats


class Executor:
    def __init__(
        self, client: PingClient, handler: ResponseHandler, logger: StatsLogger
    ) -> None:
        self._client = client
        self._handler = handler
        self._logger = logger

    def execute(self, url: str, nb_tries: int):
        stats = PingStats()

        for _ in range(0, nb_tries):
            response = self._client.ping(url)

            self._handler.update_stats(stats, response)
            self._logger.show_current_stats(stats)

        self._logger.show_stats_summary(stats)
