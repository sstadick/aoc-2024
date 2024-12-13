import sys
from enum import Enum, auto
from typing import Iterable, List


class Direction(Enum):
    Increase = auto()
    Decrease = auto()
    Same = auto()

    def get_direction(left: int, right: int) -> "Direction":
        if left < right:
            return Direction.Increase
        elif left == right:
            return Direction.Same
        else:
            return Direction.Decrease


def is_safe(report: Iterable[int]) -> bool:
    previous = None
    direction = None
    is_safe = True

    for level in report:
        if previous is None and direction is None:
            previous = level
        elif previous is None and direction is not None:
            # Could remove this case entirely to speed up
            print("Can't have a previous direction and no previous value")
            exit(1)
        elif previous is not None and direction is None:
            new_dir = Direction.get_direction(previous, level)
            diff = abs(previous - level)
            # if diff not in (1, 2, 3):
            if not (diff > 0 and diff < 4):
                is_safe = False
                break
            direction = new_dir
            previous = level
        elif previous is not None and direction is not None:
            # Could just be an else to speed up.
            new_dir = Direction.get_direction(previous, level)
            diff = abs(previous - level)
            if not (diff > 0 and diff < 4) or direction != new_dir:
                is_safe = False
                break
            direction = new_dir
            previous = level

    return is_safe


def main():
    file = sys.argv[1]
    safe_count = 0
    with open(file, "r") as fh:
        for line in fh:
            report = (int(level) for level in line.split())
            safe_count += int(is_safe(report))
    print(safe_count)


if __name__ == "__main__":
    main()
