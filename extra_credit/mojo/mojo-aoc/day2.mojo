import sys

fn main() raises:
    var input_file = sys.arg.argv()[1]
    var bytes = open(input_file, 'r').read_bytes()


    print(len(bytes))
