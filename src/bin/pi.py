#!/usr/bin/env python3

import random
import math


def pi_calc_stock(samples=100000):
    count_inside = 0
    for count in range(samples):
        d = math.hypot(random.random(), random.random())
        if d < 1:
            count_inside += 1
    return 4.0 * count_inside / samples


def pi_calc_nump(samples=100000000):
    import numpy as np
    count_inside = 0
    for count in range(samples):
        d = np.hypot(np.random.rand(), np.random.rand())
        if d < 1:
            count_inside += 1
    return 4.0 * count_inside / samples


print(pi_calc_stock())
