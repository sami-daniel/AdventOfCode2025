# frozen_string_literal: true

def process_line(line)
  stack = []
  line.each_char.with_index do |char, i|
    stack.pop while !stack.empty? && stack[-1] < char && (stack.length + (line.length - i) > 12)
    stack.push(char) if stack.length <= 12
  end

  stack.join
end

input = ARGV[0]
file_content = File.read(input)
lines = file_content.split("\n")
total_joltage = 0

lines.each do |line|
  total_joltage += process_line(line).to_i
end

puts total_joltage
