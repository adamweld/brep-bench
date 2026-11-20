+++
title = "A post and a block tangent to it on three sides and cutting across it on the fourth, their common part"
tags = ["tangent", "boolean", "common", "analytic", "cylinder", "planar", "coincident"]
origin = "OCCT tests/boolean/bopcommon_simple/R3"

[volume]
value = 631.8519510713537
exact = "V = 10·S, S = 50π/3 + 25√3/4 the area of the disc of radius 5 below the chord y = 5/2 (the disc less the cap 25π/3 − 25√3/4 above it)"
tex = '10\left(\tfrac{50\pi}{3} + \tfrac{25\sqrt3}{4}\right) = \tfrac{500\pi}{3} + \tfrac{125\sqrt3}{2}'

[area]
value = 422.4124408320341
exact = "two ends of S, the wall over the arc of angle 4π/3 (length 20π/3) 10 high, and the flat face over the chord of length 5√3, 10 high: 2S + 200π/3 + 50√3"
tex = '100\pi + \tfrac{125\sqrt3}{2}'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −5 to 5, y from −5 to 5/2 and z from 0 to 10: it is as tall as the post, its top and bottom faces level with the post's. Make the solid common to both.

The block's faces x = −5, x = 5 and y = −5 each touch the post along a line of its wall. Its face y = 5/2 cuts across the post, a chord 5√3 long at distance 5/2 from the axis, so the axis is inside the block; the block's corners (±5, −5) and (±5, 5/2) are all outside the post.

The common part is the post with the segment above y = 5/2 taken off: a prism 10 high on the disc of radius 5 cut by the line y = 5/2, keeping the larger part.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cap \bigl([-5,5]\times[-5,\tfrac52]\times[0,10]\bigr)$$
