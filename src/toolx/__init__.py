import sys

from toolx._core import run_cli


def main() -> None:
    raise SystemExit(run_cli(sys.argv))
