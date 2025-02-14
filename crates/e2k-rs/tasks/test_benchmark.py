import e2k
import pytest


def test_c2k(benchmark):
    c2k = e2k.C2K()

    def bench_c2k():
        c2k("constants")

    benchmark.pedantic(bench_c2k, iterations=1, rounds=100)


def test_p2k(benchmark):
    p2k = e2k.P2K()

    def bench_p2k():
        p2k(["K", "AA1", "N", "S", "T", "AH0", "N", "T", "S"])  # type:ignore

    benchmark.pedantic(bench_p2k, iterations=1, rounds=100)


if __name__ == "__main__":
    pytest.main(["-v", __file__])
