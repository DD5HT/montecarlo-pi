#!/usr/bin/env ruby
count_inside = 0

for count in 0..100_000_000
    d = Math.hypot(rand, rand)
    if d < 1 
        count_inside += 1
    end
end
count += 1
puts "π ~ #{4.0 * count_inside / count}" 