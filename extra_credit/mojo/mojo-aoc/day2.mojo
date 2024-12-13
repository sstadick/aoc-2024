import sys
from collections import Optional
from tensor import Tensor
from time import perf_counter_ns

from ExtraMojo.fs.file import read_lines, find_chr_all_occurances
from ExtraMojo.bench.bench import Bench

alias SPACE = 32
alias NEWLINE = 10


@value
struct Direction:
    var value: UInt8
    alias Increase = Direction(0)
    alias Decrease = Direction(1)
    alias Same = Direction(2)

    @staticmethod
    fn get_direction(left: UInt8, right: UInt8) -> Direction:
        if left < right:
            return Direction.Increase
        elif left == right:
            return Direction.Same
        else:
            return Direction.Decrease

    fn __eq__(self, other: Direction) -> Bool:
        return self.value == other.value

    fn __ne__(self, other: Direction) -> Bool:
        return not (self == other)


@always_inline
fn is_digit(value: UInt8) -> Bool:
    return value >= 48 and value <= 57


@always_inline
fn is_space(value: UInt8) -> Bool:
    return value == SPACE


@always_inline
fn is_newline(value: UInt8) -> Bool:
    return value == NEWLINE


# TODO: try passing a ref again? or is it implicity one? hard to tell
fn check_safety(borrowed report: List[UInt8]) -> Bool:
    var previous = Optional[UInt8](None)
    var direction = Optional[Direction](None)
    var is_safe = True
    # for i in range(0, len(report)):
    #     var level = report[i]
    for level in report:
        if not previous and not direction:
            previous = level[]
        elif previous and not direction:
            new_dir = Direction.get_direction(previous.value(), level[])
            var diff = abs(
                previous.value().cast[DType.int16]()
                - level[].cast[DType.int16]()
            )
            if not (diff > 0 and diff < 4):
                is_safe = False
                break
            direction = new_dir
            previous = level[]
        elif previous and direction:
            # Could just be an else to speed up.
            new_dir = Direction.get_direction(previous.value(), level[])
            diff = abs(
                previous.value().cast[DType.int16]()
                - level[].cast[DType.int16]()
            )
            if not (diff > 0 and diff < 4) or direction.value() != new_dir:
                is_safe = False
                break
            direction = new_dir
            previous = level[]
    return is_safe


fn fill_report[T: DType](borrowed line: Tensor[T], inout report: List[UInt8]):
    var num: UInt8 = 0
    for i in range(line.num_elements()):
        if line[i] == SPACE:
            report.append(num)
            num = 0
            continue
        num = (num * 10) + (line[i].cast[DType.uint8]() - 48)

    # get the last element
    report.append(num)


fn read_lines_method() raises:
    var input_file = sys.arg.argv()[1]
    var answer = 0
    var report = List[UInt8]()
    for line in read_lines(input_file):
        fill_report[DType.uint8](line[], report)
        if check_safety(report):
            answer += 1
        report.clear()
    # print(answer)


fn original() raises:
    var input_file = sys.arg.argv()[1]
    var bytes = open(input_file, "r").read_bytes()
    var answer = 0
    var offset = 0

    # sleep(0.0001)

    var report = List[UInt8]()
    var level: UInt8 = 0
    while offset < len(bytes):
        if is_digit(bytes[offset]):
            level = (level * 10) + (bytes[offset] - 48)
        elif is_space(bytes[offset]):
            report.append(level)
            level = 0
        elif is_newline(bytes[offset]):
            report.append(level)
            # reset everything
            if check_safety(report):
                answer += 1
            report.clear()
            level = 0

        offset += 1

    if len(report) != 0 and check_safety(report):
        answer += 1

    print(answer)


fn main() raises:
    var bench = Bench(
        name="Day2 Part1",
        func=original,
        max_iterations=1000,
        warmup_iterations=5,
    )

    bench.benchmark()

    # for _ in range(10):
    #     start = perf_counter_ns()
    #     original()
    #     end = perf_counter_ns()
    #     print((end - start) / 1000000000.0, " s")

    # print("Timing by line")
    # time = timeit[read_lines_method]()
    # print("Took: ", time / 1000, "ms")
