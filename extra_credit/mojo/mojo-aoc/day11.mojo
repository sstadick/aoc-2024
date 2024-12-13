import sys

from ExtraMojo.bench.bench import Bench


fn process(bytes: List[UInt8]):
    print(len(bytes))
    pass


fn main() raises:
    var input_file = sys.arg.argv()[1]
    var bytes = open(input_file, "r").read_bytes()

    fn inner() raises:
        process(bytes)

    var bench = Bench(
        name="Day2 Part1",
        func=inner,
        max_iterations=1000,
        warmup_iterations=5,
    )

    bench.benchmark()

    pass
