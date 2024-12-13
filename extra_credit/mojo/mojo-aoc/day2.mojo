import sys
from collections import Optional

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


fn check_safety(report: List[UInt8]) -> Bool:
    var previous = Optional[UInt8](None)
    var direction = Optional[Direction](None)
    var is_safe = True
    for i in range(0, len(report)):
        var level = report[i]
        if not previous and not direction:
            previous = level
        elif previous and not direction:
            new_dir = Direction.get_direction(previous.value(), level)
            var diff = abs(
                previous.value().cast[DType.int16]() - level.cast[DType.int16]()
            )
            if not (diff > 0 and diff < 4):
                is_safe = False
                break
            direction = new_dir
            previous = level
        elif previous and direction:
            # Could just be an else to speed up.
            new_dir = Direction.get_direction(previous.value(), level)
            diff = abs(
                previous.value().cast[DType.int16]() - level.cast[DType.int16]()
            )
            if not (diff > 0 and diff < 4) or direction.value() != new_dir:
                is_safe = False
                break
            direction = new_dir
            previous = level
    return is_safe


fn main() raises:
    var input_file = sys.arg.argv()[1]
    var bytes = open(input_file, "r").read_bytes()
    var answer = 0
    var offset = 0

    # Vars for the level

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
