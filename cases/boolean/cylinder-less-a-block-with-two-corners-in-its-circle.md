+++
title = "A cylinder less a block with two corners inside its circle"
tags = ["boolean", "cut", "analytic", "coincident", "planar"]
origin = "OCCT tests/boolean/bopcut_simple/ZB8"

[volume]
value = 340.9007251001004
exact = "cylinder π·4²·8 less the block's part inside it, 4·(4√3 + 8π/3): the footprint inside the circle is the slab |y| ≤ 2, x ≥ 0 of the disk, the integral from −2 to 2 of √(16 − y²) dy = 4√3 + 8π/3"
tex = '128\pi - 4\left(4\sqrt3 + \tfrac{8\pi}{3}\right)'

[area]
value = 328.55054684657665
exact = "cylinder 96π; the base's loss and the cut floor's gain at z = 4 are the same footprint area; the wall loses 60° of its circle (two arcs of 30°), (π/3)·4·4 = 16π/3; the new walls are x = 0 (4 × 4) and y = ±2 for x from 0 to √12 (2·4·√12)"
tex = '96\pi - \tfrac{16\pi}{3} + 16 + 16\sqrt3'
+++

A cylinder of radius 4 and height 8 standing on the xy plane with its axis the z axis, and a block from (0, −2, 0) to (6, 2, 4): half the cylinder's height, its base in the cylinder's base. Cut the block away from the cylinder.

The block's face x = 0 passes through the axis. Its corners (0, ±2) lie inside the cylinder's circle and its corners (6, ±2) outside it. Its faces y = ±2 cut across the circle, and its face x = 6 lies clear of the cylinder.

**As a set:**

$$\{\rho \le 4,\ 0 \le z \le 8\} \setminus \bigl([0,6]\times[-2,2]\times[0,4]\bigr)$$
