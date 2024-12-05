def main
  lhs = []
  rhs = []
  File.foreach('../../day-01/input1_bigger.txt') do |line|
    parts = line.split
    lhs << parts[0].to_i
    rhs << parts[1].to_i
  end
  
  result = lhs.sort.zip(rhs.sort).sum { |v| (v[0] - v[1]).abs }
  puts result
end

def main_chat
  # Get file size to pre-allocate arrays efficiently
  file_size = File.size('../../day-01/input1_bigger.txt')
  estimated_lines = file_size / 16  # Rough estimate of average line length
  
  left = Array.new(estimated_lines)
  right = Array.new(estimated_lines)
  i = 0
  
  # Single pass through file with minimal allocations
  File.open('../../day-01/input1_bigger.txt') do |f|
    f.each_line do |line|
      # Faster than split for exactly 2 numbers
      first_space = line.index(' ')
      left[i] = line[0...first_space].to_i
      right[i] = line[first_space + 3..-1].to_i
      i += 1
    end
  end

  # Trim arrays to actual size
  left.compact!
  right.compact!
  
  # Calculate result
  result = left.sort!.zip(right.sort!).sum { |a, b| (a - b).abs }
  puts result
end

main_chat if $PROGRAM_NAME == __FILE__