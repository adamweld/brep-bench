+++
title = "A cylinder less a block with two faces tangent to it"
tags = ["boolean", "cut", "analytic", "tangent", "coincident", "planar"]
origin = "OCCT tests/boolean/bopcut_simple/ZC6"

[volume]
value = 340.9007251001004
exact = "cylinder 128π less 4·A, A the disk of radius 4 within 0 ≤ x ≤ 2: twice the integral from 0 to 2 of √(16 − x²) dx = 4√3 + 8π/3"
tex = '128\pi - 4\left(4\sqrt3 + \tfrac{8\pi}{3}\right)'

[area]
value = 344.55054684657665
exact = "cylinder 96π; the base's loss and the cut floor's gain at z = 4 cancel; the wall loses the two arcs from 60° to 90° and from −90° to −60°, together (π/3)·4·4 = 16π/3; the new walls are x = 0 (8 × 4) and x = 2 for |y| ≤ √12 (2·√12·4); the tangent faces y = ±4 touch the cylinder along a line and add nothing"
tex = '96\pi - \tfrac{16\pi}{3} + 32 + 16\sqrt3'
+++

A cylinder of radius 4 and height 8 standing on the xy plane with its axis the z axis, and a block from (0, −4, 0) to (2, 4, 4): half the cylinder's height, its base in the cylinder's base. Cut the block away from the cylinder.

The block's faces y = ±4 are tangent to the cylinder, along the vertical lines through (0, ±4), which are the block's own edges. Its face x = 0 passes through the axis and its face x = 2 cuts across the circle. The block's corners (0, ±4) lie on the cylinder's wall and its corners (2, ±4) outside it.

**As a set:**

$$\{\rho \le 4,\ 0 \le z \le 8\} \setminus \bigl([0,2]\times[-4,4]\times[0,4]\bigr)$$
