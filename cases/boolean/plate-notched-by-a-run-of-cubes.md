+++
title = "A plate notched along an edge by a run of seven cubes"
tags = ["boolean", "cut", "analytic", "sequence", "planar"]
origin = "OCCT boolean/simplify/A4"

[volume]
value = 2004.0
exact = "60·10·4 − 4·3·(15 + 6 + 6 + 6): four notches 3 deep and 4 thick, of widths 15, 6, 6, 6"
tex = '60\cdot 10\cdot 4 - 4\cdot 3\,(15 + 6 + 6 + 6)'

[area]
value = 1658
exact = "2(600 + 240 + 40) + Σ(24 − 6w), w = 15, 6, 6, 6"
tex = '2(600 + 240 + 40) + \sum_w (24 - 6w),\quad w = 15, 6, 6, 6'
+++

A plate from the origin to (60, 10, 4). Cut it seven times, one cut after another, with a cube of side 6 each: the cube centered at x = c, y = 0, z = 2, that is x from c − 3 to c + 3, y from −3 to 3, z from −1 to 5, for c = 8, 11, 14, 17, 28, 40, 52 in that order. The first four overlap one another and make one notch from x = 5 to x = 20; the other three stand apart, from x = 25 to 31, from 37 to 43 and from 49 to 55. Each notch is 3 deep into the plate and runs through its thickness, so the plate keeps four notches on its front long edge, y = 0.

**As a set:**

$$\bigl([0,60]\times[0,10]\times[0,4]\bigr) \setminus \bigcup_c \{\,|x - c| \le 3,\ y \le 3\,\}$$
