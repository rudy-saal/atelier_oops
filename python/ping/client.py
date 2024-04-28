from abc import ABC, abstractmethod
import requests

from ping.response import PingResponse


class PingClient(ABC):
    @abstractmethod
    def ping(self, url: str) -> PingResponse:
        pass


class HttpPingClient(PingClient):
    def ping(self, url: str) -> PingResponse:
        response = requests.get(url)

        return PingResponse(status=response.status_code)
