+++
title = "A cone shrunk by 1"
tags = ["offset", "shrink", "cone", "apex", "analytic"]
origin = "OCCT tests/offset/shape_type_i/D3"

[volume]
value = 1.3962634015954636
exact = "πR²H/3, R = 1, H = 4/3"
tex = '\tfrac{1}{3}\pi R^2 H,\quad R = 1,\ H = \tfrac{4}{3}'

[area]
value = 8.377580409572783
exact = "πR² + πRL = π + 5π/3, L = 5/3: the base and the side"
tex = '\pi R^2 + \pi R L,\quad R = 1,\ L = \tfrac{5}{3}'
+++

A cone on the z axis with a base of radius 3 at z = 0 and its apex at (0, 0, 4), whose side leans in
3 for every 4 it rises (half-angle α = atan(3/4) at the apex).

Shrink it by 1: the solid of every point at least 1 from the cone's outside.

The side moves in to the cone ρ + ¾z = 7/4, the same cone shifted 5/4 along its normals inward, with
its apex at (0, 0, 7/3); the base moves up to z = 1. They meet along the circle of radius 1 at z =
1, so the result is the cone with its base of radius 1 at z = 1 and its apex at (0, 0, 7/3).

**As a set:**

$$\{\, \rho \le \tfrac34 \bigl(\tfrac{7}{3} - z\bigr),\ z \ge 1 \,\}$$
