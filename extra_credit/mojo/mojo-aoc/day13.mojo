import math
import sys
from builtin.tuple import Tuple
from collections import Optional
from tensor import Tensor

from ExtraMojo.bench.bench import Bench


@always_inline
fn is_ascii_num(byte: UInt8) -> Bool:
    if byte >= 48 and byte <= 57:
        return True
    else:
        return False


@value
struct NumParser[T: DType = DType.uint64]:
    var parsed_num: Optional[Scalar[T]]
    var bytes_read: Int

    @always_inline
    @staticmethod
    fn try_parse_num(
        borrowed bytes: Tensor[DType.uint8], starting_offset: Int = 0
    ) -> Self:
        var offset = starting_offset
        var num: Scalar[T] = 0

        while offset < bytes.num_elements() and is_ascii_num(bytes[offset]):
            num = (num * 10) + (bytes[offset].cast[T]() - 48)
            offset += 1

        if starting_offset == offset:
            return Self(None, 0)
        return Self(num, offset - starting_offset)


@value
struct MachineIterator:
    """Iterator over Machines."""

    var offset: Int
    var data: Tensor[DType.uint8]
    var len: Int

    fn __init__(inout self, owned data: Tensor[DType.uint8]):
        self.offset = 0
        self.data = data^
        # Length is only checked (in the for-in loop) for 0. So as long as we can set to 0 before
        # the last iteration we don't actually need to have a full size. I'm sure this breaks some
        # contracts in the Sizeable trait.
        self.len = 1

    fn __len__(self) -> Int:
        """Mojo uses the __len__ to tell how many elements remain in the iterator.
        """
        return self.len

    fn __iter__(self) -> Self:
        return self

    fn __next__(inout self) raises -> Machine:
        # TODO: tuple unpacking?
        var button_a = Self._parse_button(self.data, self.offset)
        self.offset += button_a[2] + 1

        var button_b = Self._parse_button(self.data, self.offset)
        self.offset += button_b[2] + 1

        var prize = Self._parse_prize(self.data, self.offset)
        self.offset += prize[2] + 2

        if self.offset >= self.data.num_elements():
            self.len = 0

        return Machine(
            ax=button_a[0],
            ay=button_a[1],
            bx=button_b[0],
            by=button_b[1],
            px=prize[0],
            py=prize[1],
        )

    @staticmethod
    fn _parse_button(
        borrowed bytes: Tensor[DType.uint8], starting_offset: Int = 0
    ) raises -> Tuple[UInt64, UInt64, Int]:
        var offset = 12 + starting_offset
        var num_result = NumParser.try_parse_num(bytes, offset)
        if not num_result.parsed_num:
            raise Error("Failed to parse X from Button")
        var x = num_result.parsed_num.value()
        offset += num_result.bytes_read + 4

        num_result = NumParser.try_parse_num(bytes, offset)
        if not num_result.parsed_num:
            raise Error("Failed to parse Y from Button")
        var y = num_result.parsed_num.value()
        offset += num_result.bytes_read
        return (x, y, offset - starting_offset)

    @staticmethod
    fn _parse_prize(
        borrowed bytes: Tensor[DType.uint8], starting_offset: Int = 0
    ) raises -> Tuple[UInt64, UInt64, Int]:
        var offset = 9 + starting_offset
        var num_result = NumParser.try_parse_num(bytes, offset)
        if not num_result.parsed_num:
            raise Error("Failed to parse X from Button")
        var x = num_result.parsed_num.value()
        offset += num_result.bytes_read + 4

        num_result = NumParser.try_parse_num(bytes, offset)
        if not num_result.parsed_num:
            raise Error("Failed to parse Y from Button")
        var y = num_result.parsed_num.value()
        offset += num_result.bytes_read
        return (x, y, offset - starting_offset)


@value
struct Machine:
    var ax: UInt64
    var ay: UInt64
    var bx: UInt64
    var by: UInt64
    var px: UInt64
    var py: UInt64

    @staticmethod
    fn parse_machines(
        owned data: Tensor[DType.uint8],
    ) -> MachineIterator:
        return MachineIterator(data^)

    fn __str__(self) -> String:
        return (
            "("
            + str(self.ax)
            + ", "
            + str(self.ay)
            + "), "
            + "("
            + str(self.bx)
            + ", "
            + str(self.by)
            + "), "
            + "("
            + str(self.px)
            + ", "
            + str(self.py)
            + ")"
        )


fn process(owned bytes: Tensor[DType.uint8]) raises:
    var total: UInt64 = 0
    for m in Machine.parse_machines(bytes^):
        var ax = m.ax.cast[DType.float64]()
        var ay = m.ay.cast[DType.float64]()
        var bx = m.bx.cast[DType.float64]()
        var by = m.by.cast[DType.float64]()
        var px = m.px.cast[DType.float64]()
        var py = m.py.cast[DType.float64]()

        var ca = (px * by - py * bx) / (ax * by - ay * bx)
        var cb = (px - ax * ca) / bx
        if (ca % 1.0 == 0.0) and (cb % 1.0 == 0.0):
            total += ((round(ca) * 3.0) + round(cb)).cast[DType.uint64]()
    # print(total)


fn main() raises:
    var input_file = sys.arg.argv()[1]
    var bytes = Tensor(open(input_file, "r").read_bytes())

    fn inner() raises:
        process(bytes)

    bench = Bench(
        name="Day 13", max_iterations=10000, warmup_iterations=100, func=inner
    )
    bench.benchmark()
