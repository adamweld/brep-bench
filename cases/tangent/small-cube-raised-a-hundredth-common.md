+++
title = "A small cube and itself raised a hundredth of its side, their common part"
tags = ["tangent", "boolean", "intersection", "planar", "coincident", "small"]

[volume]
value = 3.34125e-9
exact = "s²(s − t), s = 0.0015, t = 0.000015"
tex = 's^2(s - t),\quad s = 0.0015,\ t = 0.000015'

[area]
value = 1.341e-5
exact = "2s² + 4s(s − t)"
tex = '2s^2 + 4s(s - t)'
+++

A small cube, of side 0.0015, from the origin to (0.0015, 0.0015, 0.0015), and a copy of itself
raised by 0.000015 along z, a hundredth of its side: the part common to both.

The common part is the box from (0, 0, 0.000015) to (0.0015, 0.0015, 0.0015). The four sides of the
copy lie in the planes of the cube's own sides; its bottom stands 0.000015 above the cube's. That
is a small distance, but it is 1% of the solid: a kernel that takes the copy's bottom for the
cube's own, by a tolerance that does not shrink with the model, returns the cube and is 1% out.

**As a set:**

$$[0, 0.0015] \times [0, 0.0015] \times [0.000015, 0.0015]$$
