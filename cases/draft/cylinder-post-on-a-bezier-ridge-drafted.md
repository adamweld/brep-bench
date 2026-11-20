+++
title = "A round post on a Bézier ridge, its side drafted"
tags = ["draft", "cone", "bezier", "boolean"]
settled = false
origin = "OCCT tests/draft/angle/I2"
+++

The ridge: a block standing on the square 0 ≤ x ≤ 20, 0 ≤ y ≤ 20 of the xy plane, its floor flat at
z = 0, its four sides upright, and its top the surface swept by a cubic Bézier arc moved along y. In
the plane y = 0 the arc has control points (0, 6), (20/3, 14), (40/3, 14) and (20, 6) in (x, z), so
it is B(t) = ((20t), 6(1 − t)³ + 42t(1 − t)² + 42t²(1 − t) + 6t³) for t from 0 to 1, x = 20t. The
block's top is the arc's height above every point of the square, 6 at the sides x = 0 and x = 20
and 12 at x = 10, the same for every y: a ridge along y, its section the region under the arc.

A post stands on it: the cylinder of radius 4 about the vertical line through (10, 10), from z = 0
to z = 26, united with the block. Its side shows only above the ridge, which is 11 to 12 high over
the post's footprint.

Draft the post's side by 4°, with the pull direction +z and the neutral plane z = 0, the block's
floor: the side becomes the cone through the circle of radius 4 about (10, 10) at z = 0, leaning 4°
from the z direction, inward as it rises, its radius 4 − z·tan 4°. It still runs down into the
ridge, meeting it a little lower, and the top shrinks to meet it, to radius 4 − 26·tan 4° ≈ 2.18.
The block and the post's top are not drafted. It has no closed form that has been checked.

**As a set:**

$$\{\, 0 \le x, y \le 20,\ 0 \le z \le H(x) \,\} \cup \{\, \sqrt{(x - 10)^2 + (y - 10)^2} \le 4 - z\tan 4^\circ,\ 0 \le z \le 26 \,\}$$

where H(x) is the height of the arc above x.
