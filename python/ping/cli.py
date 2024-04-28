from dataclasses import dataclass
from argparse import ArgumentDefaultsHelpFormatter, ArgumentParser


@dataclass
class Args:
    url: str
    nb_tries: int


def parse_args() -> Args:
    parser = ArgumentParser(prog="ping", formatter_class=ArgumentDefaultsHelpFormatter)
    parser.add_argument("--url", "-u", help="URL to ping", required=True)
    parser.add_argument("--tries", "-t", type=int, default=20, help="Number of tries")

    args = parser.parse_args()

    return Args(url=args.url, nb_tries=args.tries)
