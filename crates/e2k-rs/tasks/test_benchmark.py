import e2k
import pytest


def test_c2k(benchmark):
    c2k = e2k.C2K()

    word = "constants"
    def bench_c2k():
        c2k(word)

    benchmark.pedantic(bench_c2k, iterations=1, rounds=100)

def test_c2k_long(benchmark):
    c2k = e2k.C2K(128)

    word = "phosphoribosylaminoimidazolesuccinocarboxamide"
    def bench_c2k():
        c2k(word)

    benchmark.pedantic(bench_c2k, iterations=1, rounds=100)


def test_p2k(benchmark):
    p2k = e2k.P2K()

    pron = ["K", "AA1", "N", "S", "T", "AH0", "N", "T", "S"]
    def bench_p2k():
        p2k(pron)  # type:ignore

    benchmark.pedantic(bench_p2k, iterations=1, rounds=100)


if __name__ == "__main__":
    pytest.main(["-v", __file__])
