require "option_parser"

input = ""
OptionParser.parse do |parser|
  parser.banner = "AoC Day5 Part1"
  parser.on "-v", "--version", "Show Version" do
    puts "version 1.0"
    exit
  end
  parser.on "-i INPUT_FILE", "--input=INPUT_FILE", "Input file" do |input_file|
    input = input_file
  end
end

if input.empty?
  puts "Missing input file"
  exit
end

enum Direction
  Increase
  Decrease
  Same
end

def get_direction(left : Int32, right : Int32) : Direction
  if left < right
    Direction::Increase
  elsif left > right
    Direction::Decrease
  else
    Direction::Same
  end
end

enum Safety
  Safe
  Unsafe
end

def is_safe?(report)
  prev = nil
  direction = nil
  overall_safety = Safety::Safe

  report.each do |level|
    if overall_safety == Safety::Unsafe
      return false
    end
    case {prev, direction}
    when {.nil?, .nil?} then prev = level
    when {.nil?, _}     then exit 1
    when {_, .nil?}
      new_dir = get_direction(prev, level)
      diff = (prev - level).abs
      if !(1..3).includes?(diff)
        overall_safety = Safety::Unsafe
      end
      direction = new_dir
      prev = level
    when {_, _}
      new_dir = get_direction(prev, level)
      diff = (prev - level).abs
      if !(1..3).includes?(diff) || (direction != new_dir)
        overall_safety = Safety::Unsafe
      end
      direction = new_dir
      prev = level
    end
  end
  true
end

def process(file)
  fh = File.open(file, "rb")
  count_of_safe = 0
  while line = fh.gets
    report = line.split.map { |num| num.to_i }
    safety = is_safe?(report)
    if safety
      count_of_safe += 1
    end
  end
  puts count_of_safe
end

process(input)
