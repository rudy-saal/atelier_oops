from ping.cli import parse_args
from ping.client import HttpPingClient
from ping.exec import Executor
from ping.ping_logging import ConsoleStatsLogger
from ping.response import ResponseHandler


client = HttpPingClient()
handler = ResponseHandler()
logger = ConsoleStatsLogger()
executor = Executor(client, handler, logger)

args = parse_args()

executor.execute(args.url, args.nb_tries)
