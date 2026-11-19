+++
title = "Two cylinders a few microns across, crossed"
tags = ["boolean", "common", "cylinder", "analytic", "ssi", "small", "scale"]
origin = "brep-conformance 1.0.0, booleans/scale/microns-to-metres-equal-cylinders"

[volume]
value = 1.44e-07
exact = "16r³/3, r = 0.003"
tex = '\tfrac{16}{3}r^3,\quad r = 0.003'

[area]
value = 0.000144
exact = "16r², r = 0.003"
tex = '16r^2,\quad r = 0.003'
+++

The solid common to two cylinders of radius 0.003 and length 0.02 that cross at right angles at
the origin: a Steinmetz bicylinder a few microns across, if a unit is a millimetre. The first runs
along the x axis from x = −0.01 to x = 0.01, the second along the y axis from y = −0.01 to
y = 0.01. Neither cylinder's end is reached.

The two walls cross in two ellipses that meet each other at (0, 0, ±0.003), where the curve of
intersection is singular.

**As a set:**

$$\{\, y^2 + z^2 \le 0.003^2,\ |x| \le 0.01 \,\} \cap \{\, x^2 + z^2 \le 0.003^2,\ |y| \le 0.01 \,\}$$
