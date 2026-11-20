+++
title = "A cone grown by 1, its edges kept sharp"
tags = ["offset", "grow", "sharp-edge", "cone", "apex", "analytic"]
origin = "OCCT tests/offset/shape_type_i/D2"

[volume]
value = 174.53292519943295
exact = "πR²H/3, R = 5, H = 20/3"
tex = '\tfrac{1}{3}\pi R^2 H,\quad R = 5,\ H = \tfrac{20}{3}'

[area]
value = 209.43951023931956
exact = "πR² + πRL = 25π + 125π/3, L = 25/3: the base and the side"
tex = '\pi R^2 + \pi R L,\quad R = 5,\ L = \tfrac{25}{3}'
+++

A cone on the z axis with a base of radius 3 at z = 0 and its apex at (0, 0, 4), whose side leans in
3 for every 4 it rises (half-angle α = atan(3/4) at the apex).

Grow it by 1 with its edges kept sharp: move each face 1 outward along its normal, and extend the
moved faces until they meet, so that no round is added and every edge stays where its two faces
meet.

The side moves out to the cone ρ + ¾z = 17/4, the same cone shifted 5/4 along its normals, with its
apex at (0, 0, 17/3); the base moves down to z = −1. They meet along the circle of radius 5 at z =
−1, so the result is the cone with its base of radius 5 at z = −1 and its apex at (0, 0, 17/3): the
apex stays a point and the rim stays a sharp edge.

**As a set:**

$$\{\, \rho \le \tfrac34 \bigl(\tfrac{17}{3} - z\bigr),\ z \ge -1 \,\}$$
