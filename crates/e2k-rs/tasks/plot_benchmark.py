import json
import matplotlib.pyplot as plt

python_benchmark = json.load(open("./benchmark.json"))
rust_c2k_benchmark = json.load(open("../../../target/criterion/c2k/new/sample.json"))
rust_p2k_benchmark = json.load(open("../../../target/criterion/p2k/new/sample.json"))

python_benchmarks = python_benchmark["benchmarks"]

# unit: s/iter
python_c2k_benchmarks = [b for b in python_benchmarks if b["name"] == "test_c2k"][0][
    "stats"
]["data"]
python_p2k_benchmarks = [b for b in python_benchmarks if b["name"] == "test_p2k"][0][
    "stats"
]["data"]

# unit: ns/iter
rust_c2k_benchmarks = rust_c2k_benchmark["times"]
rust_p2k_benchmarks = rust_p2k_benchmark["times"]

fig = plt.figure(figsize=(12, 6))

c2k_hist_fig = fig.add_subplot(2, 2, 1)
c2k_hist_fig.hist(
    list(map(lambda x: x * 1000, python_c2k_benchmarks)),
    bins=10,
    alpha=0.5,
    label="Python C2K",
)
c2k_hist_fig.hist(
    list(map(lambda x: x / 1000 / 1000, rust_c2k_benchmarks)),
    bins=10,
    alpha=0.5,
    label="Rust C2K",
)
c2k_hist_fig.yaxis.set_major_formatter(plt.FuncFormatter(lambda x, _: f"{x:.0f}"))
c2k_hist_fig.set_title("C2K")
c2k_hist_fig.legend()

c2k_fig = fig.add_subplot(2, 2, 2)
c2k_fig.scatter(
    range(len(python_c2k_benchmarks)),
    list(map(lambda x: x * 1000, python_c2k_benchmarks)),
    label="Python C2K",
)
c2k_fig.scatter(
    range(len(rust_c2k_benchmarks)),
    list(map(lambda x: x / 1000 / 1000, rust_c2k_benchmarks)),
    label="Rust C2K",
)
c2k_fig.yaxis.set_major_formatter(plt.FuncFormatter(lambda x, _: f"{x:.0f}ms"))
c2k_fig.set_title("C2K")
c2k_fig.legend()

p2k_hist_fig = fig.add_subplot(2, 2, 3)
p2k_hist_fig.hist(
    list(map(lambda x: x * 1000, python_p2k_benchmarks)),
    bins=10,
    alpha=0.5,
    label="Python P2K",
)
p2k_hist_fig.hist(
    list(map(lambda x: x / 1000 / 1000, rust_p2k_benchmarks)),
    bins=10,
    alpha=0.5,
    label="Rust P2K",
)
p2k_hist_fig.yaxis.set_major_formatter(plt.FuncFormatter(lambda x, _: f"{x:.0f}"))
p2k_hist_fig.set_title("P2K")
p2k_hist_fig.legend()

p2k_fig = fig.add_subplot(2, 2, 4)
p2k_fig.scatter(
    range(len(python_p2k_benchmarks)),
    list(map(lambda x: x * 1000, python_p2k_benchmarks)),
    label="Python P2K",
)
p2k_fig.scatter(
    range(len(rust_p2k_benchmarks)),
    list(map(lambda x: x / 1000 / 1000, rust_p2k_benchmarks)),
    label="Rust P2K",
)
p2k_fig.yaxis.set_major_formatter(plt.FuncFormatter(lambda x, _: f"{x:.0f}ms"))
p2k_fig.set_title("P2K")
p2k_fig.legend()

plt.show()
