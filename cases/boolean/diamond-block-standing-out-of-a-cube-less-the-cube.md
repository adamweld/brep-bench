+++
title = "A block turned to a diamond, standing out of a cube, less the cube"
tags = ["boolean", "cut", "analytic", "planar", "turned", "coincident"]
origin = "OCCT tests/boolean/boptuc_simple/P5"

[volume]
value = 1750.0
exact = "V = 200·10 − 50·5: the block (section area 200, height 10) less its part inside the cube, a triangle of area 50 in plan over height 5"
tex = '200\cdot 10 - 50\cdot 5'

[area]
value = 994.97474683058327
exact = "area = 200 (block top) + 150 (block bottom outside the cube) + 3·100√2 (three of its sides, whole) + 50√2 (the fourth, above the cube) + 3·50 (the cube's two side faces and its top face, where the block was): 500 + 350√2"
tex = '500 + 350\sqrt2'
+++

A cube with x, y and z from 0 to 10.

A block whose section is a square of side 10√2 turned 45° about the vertical, so that it is a diamond: its vertical edges are at (0, 10), (10, 0), (20, 10) and (10, 20), and it runs from z = 5 to z = 15. Cut the cube away from the block.

Two of the block's vertical edges, at (0, 10) and (10, 0), lie on the cube's vertical edges, from z = 5 to z = 10, and the block's face between them runs through the cube along the plane x + y = 10. The block's centre line, through (10, 10), is the cube's fourth vertical edge, so the cube covers the half of the block on that side of the plane, from z = 5 to z = 10. What is left is the block's top 5, all of it, and, below that, the half of the block beyond the cube's sides x = 10 and y = 10. It is one piece.

**As a set:**

$$\bigl(\{\,|x-10| + |y-10| \le 10\,\}\times[5,15]\bigr) \setminus [0,10]^3$$
