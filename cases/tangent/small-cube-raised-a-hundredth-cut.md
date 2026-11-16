+++
title = "A small cube less itself raised a hundredth of its side"
tags = ["tangent", "boolean", "cut", "planar", "coincident", "small", "thin"]

[volume]
value = 3.375e-11
exact = "s²t, s = 0.0015, t = 0.000015"
tex = 's^2 t,\quad s = 0.0015,\ t = 0.000015'

[area]
value = 4.59e-6
exact = "2s² + 4st"
tex = '2s^2 + 4st'
+++

A small cube, of side 0.0015, from the origin to (0.0015, 0.0015, 0.0015), less a copy of itself
raised by 0.000015 along z, a hundredth of its side.

What is left is the slab under the copy: the box from the origin to (0.0015, 0.0015, 0.000015), a
hundredth as thick as it is wide. The four sides of the copy lie in the planes of the cube's own
sides, and its bottom is the slab's top. A kernel that takes the copy's bottom for the cube's
own, by a tolerance that does not shrink with the model, finds nothing left.

**As a set:**

$$[0, 0.0015] \times [0, 0.0015] \times [0, 0.000015]$$
