class PingStats:
    def __init__(self) -> None:
        self._nb_successes = 0
        self._nb_fails = 0

    def add_success(self):
        self._nb_successes += 1

    def add_fail(self):
        self._nb_fails += 1

    @property
    def total(self) -> int:
        return self._nb_successes + self._nb_fails

    @property
    def success_rate(self) -> float:
        if self.total == 0:
            return 0.0

        return self._nb_successes / self.total
