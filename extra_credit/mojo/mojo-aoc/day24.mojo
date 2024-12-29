import math
import sys
from builtin.tuple import Tuple
from collections import Optional, Dict
from tensor import Tensor

from ExtraMojo.bench.bench import Bench

import benchmark


@value
struct GateKind:
    alias AND = 0
    alias OR = 1
    alias XOR = 2

    @staticmethod
    fn parse_gate(
        read data: Tensor[DType.uint8], offset: Int
    ) raises -> (Int, Int):
        if data[offset] == ord("A"):
            return (3, GateKind.AND)
        elif data[offset] == ord("X"):
            return (3, GateKind.XOR)
        elif data[offset] == ord("O"):
            return (2, GateKind.OR)
        else:
            raise "Unable to parse Gate from input"

    @staticmethod
    fn stringify(value: Int) -> String:
        if value == GateKind.AND:
            return "AND"
        elif value == GateKind.OR:
            return "OR"
        # elif value == GateKind.XOR:
        else:
            return "XOR"

    @staticmethod
    fn apply(gate: Int, lhs: UInt8, rhs: UInt8) -> UInt8:
        if gate == GateKind.AND:
            return lhs & rhs
        elif gate == GateKind.OR:
            return lhs | rhs
        else:
            # elif gate == GateKind.XOR:
            return lhs ^ rhs


@value
struct Wire:
    # TODO: better system for ident
    var ident: String
    var value: Optional[UInt8]

    fn __init__(out self, owned ident: String, value: UInt8):
        self.ident = ident^
        self.value = value

    fn __str__(self) -> String:
        return (
            "Wire { ident: "
            + self.ident
            + ", value: "
            + str(self.value.or_else(3))
            + "}"
        )

    @staticmethod
    fn parse_wire(
        read data: Tensor[DType.uint8],
        offset: Int,
        mut wire_lookup: Dict[String, Wire],
    ) raises -> (Int, Self):
        """Take 3 characters out of the input data.

        Returns:
            Bytes read, and an instance of Wire.
        """
        # TODO: Switch to a UInt8 ident and pass in a lookup
        var bytes_read = 0
        var buffer = List[UInt8]()
        while bytes_read < 3:
            if offset + bytes_read < data.num_elements():
                buffer.append(data[offset + bytes_read])
                bytes_read += 1
            else:
                raise "Ran out of elements."

        # Strings must have a an extra byte to pretend to be null terminated
        buffer.append(0)
        return (bytes_read, Wire(String(buffer^), 0))


@value
struct Equation:
    var lhs: Wire
    var rhs: Wire
    var result: Wire
    var gate: Int

    fn __str__(read self) -> String:
        return (
            self.lhs.ident
            + " "
            + GateKind.stringify(self.gate)
            + " "
            + self.rhs.ident
            + " -> "
            + self.result.ident
        )

    fn solve(read self, mut known_values: Dict[String, UInt8]) -> Bool:
        """Returns True if solved, False if not (including if it's already solved).
        """
        # print("Solving ", str(self))
        if known_values.get(self.result.ident):
            # Already solved
            # print("Already solved")
            return False

        # get returns a copy, __getitem__ returns a ref
        var lhs = known_values.get(self.lhs.ident)
        var rhs = known_values.get(self.rhs.ident)
        if lhs and rhs:
            # Solve and update the known_values
            known_values[self.result.ident] = GateKind.apply(
                self.gate, lhs.value(), rhs.value()
            )
            # print("Got new answer")
            return True
        else:
            # Not yet solve-able
            # print("Not yet solve-able")
            return False


fn process(data: Tensor[DType.uint8]) raises:
    # Easy way to process this is to just keep making passes
    # of the equations till they all solve.
    equations, known_values = parse(data)

    # We are going to do this the dumb way, just loop over my equations and solve any
    # that are solvable.
    var any_solved = True
    while any_solved:
        any_solved = False
        for equation in equations:
            if equation[].solve(known_values):
                any_solved = True

    var z_keys = List[String]()
    for kvs in known_values.items():
        if not kvs[].key.startswith("z"):
            continue
        z_keys.append(kvs[].key)
    sort(z_keys)
    var answer: UInt64 = 0
    for key in reversed(z_keys):
        # print(key[], known_values[key[]])
        answer = answer << 1
        answer = answer | known_values[key[]].cast[DType.uint64]()

    print(answer)


fn parse(
    data: Tensor[DType.uint8],
) raises -> (List[Equation], Dict[String, UInt8]):
    # Data looks like
    # y04: 1
    #
    # ntg XOR fgs -> mjb

    var wire_lookup = Dict[String, Wire]()

    # First parse the initial values.
    var known_values = Dict[String, UInt8]()
    var offset = 0
    while data[offset] != ord("\n"):
        bytes_read, wire = Wire.parse_wire(data, offset, wire_lookup)
        # Advance past parsed bytes and the ': '
        offset += bytes_read + 2
        wire.value = data[offset] - 48
        # Advance past value and the newline
        offset += 2
        known_values[wire.ident] = wire.value.value()

    # Advance past the empty line
    offset += 1

    # Next parse the logic
    var equations = List[Equation]()
    while offset < data.num_elements():
        bytes_read, lhs_wire = wire.parse_wire(data, offset, wire_lookup)
        # print(str(lhs_wire))
        offset += bytes_read + 1
        bytes_read, gate = GateKind.parse_gate(data, offset)
        offset += bytes_read + 1
        bytes_read, rhs_wire = wire.parse_wire(data, offset, wire_lookup)
        offset += bytes_read + 4
        bytes_read, result_wire = wire.parse_wire(data, offset, wire_lookup)
        offset += bytes_read + 1

        lhs_wire.value = known_values.get(lhs_wire.ident)
        rhs_wire.value = known_values.get(rhs_wire.ident)

        equations.append(Equation(lhs_wire, rhs_wire, result_wire, gate))
        # print(
        #     str(lhs_wire),
        #     GateKind.stringify(gate),
        #     str(rhs_wire),
        #     " -> ",
        #     str(result_wire),
        # )

    return (equations, known_values)


fn main() raises:
    var input_file = sys.arg.argv()[1]
    var bytes = Tensor(open(input_file, "r").read_bytes())

    @parameter
    fn inner() raises:
        process(bytes)

    # var bench = Bench(
    #     name="Day 24", max_iterations=10000, warmup_iterations=100
    # )
    # bench.benchmark[inner]()
    inner()
    # var report = benchmark.run[inner]()
    # report.print()
