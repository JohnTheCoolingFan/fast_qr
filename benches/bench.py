import enum
import numbers
import os
import subprocess

import re


cur_dir = os.getcwd()
toml_path = os.path.join(cur_dir, "Cargo.toml")

if not os.getcwd().endswith("fast_qr") or not os.path.exists(toml_path):
    toml_path = "../Cargo.toml"

# Dry-run to see the compilation output
try:
    cargo_bench = subprocess.run(
        ["cargo", "bench", "--manifest-path", toml_path, "--no-run"]
    )
except Exception as e:
    print("An error occurred while building cargo bench")
    print(e)
    exit(1)

try:
    cargo_bench = subprocess.check_output(
        ["cargo", "bench", "--manifest-path", toml_path], stderr=subprocess.STDOUT
    )
except Exception as e:
    print("An error occurred while running cargo bench")
    print(e)
    exit(1)


lines = list(map(lambda x: x.decode("utf-8"), cargo_bench.splitlines()))

analyze_regex = re.compile(r"^Benchmarking (.*): Analyzing$", re.M | re.I)
benchmark_names = analyze_regex.findall("\n".join(lines))

time_lines = []

for line in lines:
    for benchmark in benchmark_names:
        if line.startswith(benchmark):
            time_lines.append(line)
            break

time_lines.sort()

# print("\n".join(time_lines))

number = re.compile(r"(\d+(?:\.\d*)?\s*(?:ms|ns|µs|s))")

def print_bench(benchmark_name: str, lower: str, estimate: str, upper: str, ratio=" " * len('fast_qr is 10.16x faster')):
    print(
        f"| {benchmark_name:<24} | {lower:<9} | {estimate:<9} | {upper:<9} | {ratio:<24} |"
    )

print_bench("Benchmark", "Lower", "Estimate", "Upper", "Ratio")
print_bench(":--", ":--:", ":--:", ":--:", "--")

def number_with_unit(s):
    if s.endswith("ns"):
        return float(s) * 1000000000
    if s.endswith("ms"):
        return float(s[:-2]) * 1000000
    elif s.endswith("µs"):
        return float(s[:-2]) * 1000
    elif s.endswith("s"):
        return float(s[:-1])
    else:
        raise "Not a valid number"


for i in range(0, len(time_lines), 2):
    test1, time1 = time_lines[i + 0].split("time:")
    test2, time2 = time_lines[i + 1].split("time:")

    test1, test2 = test1.strip(), test2.strip()

    data1 = number.findall(time1)
    data2 = number.findall(time2)

    data1_number = list(map(number_with_unit, data1))
    data2_number = list(map(number_with_unit, data2))

    ratio = data2_number[1] / data1_number[1]
    print_bench(test2, data2[0], data2[1], data2[2], "")
    print_bench(test1, data1[0], data1[1], data1[2], f"fast_qr is {ratio:.2f}x faster")

import platform


print(f"- System: {platform.system()}")
print(f"- Machine: {platform.machine()}")
print(f"- Processor: {platform.processor()}")


print()
print(
    "Benchmarking powered by [Criterion.rs](https://github.com/bheisler/criterion.rs). \\\n"
    "Feel free to run some benchmarkings yourself!\n\n"
)
