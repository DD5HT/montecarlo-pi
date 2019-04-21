#!/usr/bin/env python3
def pi_calc_base(samples=100_000_000):
    import random
    import math

    count_inside = 0
    for count in range(samples):
        d = math.hypot(random.random(), random.random())
        if d < 1:
            count_inside += 1
    return 4.0 * count_inside / samples


def pi_calc_numpy(samples=100_000_000):
    import numpy as np

    count_inside = 0
    for count in range(samples):
        d = np.hypot(np.random.rand(), np.random.rand())
        if d < 1:
            count_inside += 1
    return 4.0 * count_inside / samples
