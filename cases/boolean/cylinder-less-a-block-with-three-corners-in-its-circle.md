+++
title = "A cylinder less a block with three corners inside its circle"
tags = ["boolean", "cut", "analytic", "coincident", "planar"]
origin = "OCCT tests/boolean/bopcut_simple/ZC3"

[volume]
value = 366.36435332934036
exact = "cylinder 128π less 4·A, where A, the disk of radius 4 within the square [0,3]², is the integral from 0 to 3 of min(3, √(16 − x²)) dx = 3√7 + 8·asin(3/4) − 8·asin(√7/4)"
tex = '128\pi - 4\left(3\sqrt7 + 8\arcsin\tfrac34 - 8\arcsin\tfrac{\sqrt7}{4}\right)'

[area]
value = 344.75365993444785
exact = "cylinder 96π; the base's loss and the cut floor's gain at z = 4 cancel; the wall loses the arc from angle acos(3/4) to asin(3/4), of width w = asin(3/4) − acos(3/4), so 4·w·4; the new walls are x = 0 and y = 0 (3 × 4 each) and x = 3 and y = 3 for the other coordinate up to √7 (4√7 each)"
tex = '96\pi - 16\left(\arcsin\tfrac34 - \arccos\tfrac34\right) + 24 + 8\sqrt7'
+++

A cylinder of radius 4 and height 8 standing on the xy plane with its axis the z axis, and a block from (0, 0, 0) to (3, 3, 4): half the cylinder's height, its base in the cylinder's base. Cut the block away from the cylinder.

One vertical edge of the block is the cylinder's axis. Three of the block's corners, (0, 0), (3, 0) and (0, 3), lie inside the cylinder's circle; the fourth, (3, 3), lies outside it, so the block's faces x = 3 and y = 3 each leave the circle part way along their width.

**As a set:**

$$\{\rho \le 4,\ 0 \le z \le 8\} \setminus \bigl([0,3]\times[0,3]\times[0,4]\bigr)$$
