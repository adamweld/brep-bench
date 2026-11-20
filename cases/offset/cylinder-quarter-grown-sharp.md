+++
title = "A quarter of a cylinder grown by 2, its edges kept sharp"
tags = ["offset", "grow", "sharp-edge", "cylinder", "planar", "analytic"]
origin = "OCCT tests/offset/shape_type_i/F1"

[volume]
value = 1978.4901368311955
exact = "Ω(S)·(h + 2d), S = 12, d = 2, h = 8, Ω(S) = πS² − 2g + D, g = S²·acos(d/S) − d√(S² − d²), D = S²(π/4 − asin(d/S)) − d√(S² − d²) + d²: the disk of radius S less its two segments beyond x = −d and y = −d, plus the corner they share"
tex = '\Omega(S)\,(h + 2d),\quad S = 12,\ d = 2,\ h = 8'

[area]
value = 936.1399036010498
exact = "2Ω(S) + (h + 2d)(S(π/2 + 2·asin(d/S)) + 2(√(S² − d²) + d)): the top and bottom, the wall's arc, and the two flats, each running from x = −d to √(S² − d²)"
tex = '2\Omega(S) + (h + 2d)\Bigl(S\bigl(\tfrac{\pi}{2} + 2\arcsin\tfrac{d}{S}\bigr) + 2\bigl(\sqrt{S^2 - d^2} + d\bigr)\Bigr)'
+++

A quarter of a cylinder: the part of the cylinder of radius 10 about the z axis, from z = 0 to z =
8, with x ≥ 0 and y ≥ 0 (φ from 0° to 90°). Its faces are the curved wall, a flat bottom and a flat
top, each a quarter disk, and two flat cut faces, in the half-plane y = 0, x ≥ 0 and in the half-
plane x = 0, y ≥ 0, which meet along the z axis in a convex edge.

Grow it by 2 with its edges kept sharp: move each face 2 outward along its normal, and extend the
moved faces until they meet, so that no round is added and every edge stays where its two faces
meet.

The wall moves out to the radius 12, the bottom down to z = −2, the top up to z = 10, and the cut
faces to the planes y = −2 and x = −2, which meet each other along the line x = y = −2 and meet the
wall at x = 2√35 (on y = −2) and y = 2√35 (on x = −2).

**As a set:**

$$\{\, \rho \le 12,\ x \ge -2,\ y \ge -2,\ -2 \le z \le 10 \,\}$$
