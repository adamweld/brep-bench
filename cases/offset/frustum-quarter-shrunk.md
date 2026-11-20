+++
title = "A quarter of a cone frustum shrunk by 1"
tags = ["offset", "shrink", "cone", "planar"]
origin = "OCCT tests/offset/shape_type_i/F5"
settled = false
+++

A quarter of a cone frustum: the part of the cone frustum about the z axis with a base of radius 10
at z = 0 and a top of radius 6 at z = 8 (its side leans in 1 for every 2 it rises), with x ≥ 0 and y
≥ 0 (φ from 0° to 90°). Its faces are the conical side, a flat bottom and a flat top, each a quarter
of a disk, and two flat cut faces, in the half-plane y = 0, x ≥ 0 and in the half-plane x = 0, y ≥
0, which meet along the z axis in a convex edge.

Shrink it by 1: the solid of every point at least 1 from the solid's outside.

The side moves in to the cone ρ + z/2 = 10 − √5/2, the bottom up to z = 1, the top down to z = 7,
and the cut faces to the planes y = 1 and x = 1.

**As a set:**

$$\{\, \rho \le 10 - \tfrac{\sqrt5}{2} - \tfrac z2,\ x \ge 1,\ y \ge 1,\ 1 \le z \le 7 \,\}$$

Its volume and area are not worked out in closed form here, so the case is unsettled until two kernels agree.
