#!/usr/bin/env python3

import random
import math

count_inside = 0
for count in range(0, 100_000_000):
    d = math.hypot(random.random(), random.random())
    if d < 1: count_inside += 1
count += 1
print ("π ~ {}".format(4.0 * count_inside / count))
