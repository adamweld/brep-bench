+++
title = "A triangular ring segment, both its end flats moved out"
tags = ["edit", "move-face", "cone", "revolved", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/M6"

[volume]
value = 224.68722414901418
exact = "56π + 2·I, I = (4/3)(2K(7) − K(4) − K(10) + 4G(4) − 14G(7) + 10G(10)), G(ρ) = (ρ²/2)·asin(2/ρ) + w, K(ρ) = (ρ³/3)·asin(2/ρ) + ρw/3 + (4/3)·ln(ρ + w), w = √(ρ² − 4): the triangle's volume of revolution through 2π/3, 56π, and, at each distance ρ from the axis, the two extra angles asin(2/ρ) the moved flats add, over the triangle's height h(ρ) = (4/3)·min(ρ − 4, 10 − ρ)"
tex = '56\pi + 2I,\quad I = \int_4^{10}\rho\,h(\rho)\arcsin\tfrac{2}{\rho}\,d\rho = \tfrac{4}{3}\bigl(2K(7) - K(4) - K(10) + 4G(4) - 14G(7) + 10G(10)\bigr)'

[area]
value = 324.8845368223722
exact = "(8/3)(28π + 2(G(10) − G(4))) + 2·(4/3)(2S(√45) − S(√12) − S(√96) + 4√12 − 14√45 + 10√96), S(x) = x√(x² + 4)/2 + 2·asinh(x/2): the bottom, 28π + 2(G(10) − G(4)), and the two cones, together 5/3 of it (their slant is 5/3 of the radius's change); and the two moved flats, each ∫ h(√(x² + 4)) dx from x = √12 to √96"
tex = '\tfrac{8}{3}\bigl(28\pi + 2(G(10) - G(4))\bigr) + \tfrac{8}{3}\bigl(2S(\sqrt{45}) - S(\sqrt{12}) - S(\sqrt{96}) + 4\sqrt{12} - 14\sqrt{45} + 10\sqrt{96}\bigr),\quad S(x) = \tfrac{x\sqrt{x^2 + 4}}{2} + 2\operatorname{asinh}\tfrac{x}{2}'
+++

A ring segment of triangular section: the triangle with corners (4, 0, 0), (10, 0, 0) and (7, 0, 4),
in the half-plane y = 0, x ≥ 0, turned about the z axis through a third of a turn (120°), from
φ = 0° to φ = 120°, where φ is the angle about the z axis from +x toward +y. Its faces are a flat
bottom in the plane z = 0, a sector of the annulus between radii 4 and 10; two conical faces, the
inner one swept by the side from (4, 0, 0) to (7, 0, 4) and the outer one swept by the side from
(10, 0, 0) to (7, 0, 4); and two flat triangles, in φ = 0° and φ = 120°, whose outward normals are
−y and (−√3/2, −1/2, 0), the directions of decreasing and of increasing φ.

Then edit the solid directly: move each of the two flat triangles 2 outward along its outward
normal. The first, in φ = 0°, goes to the plane y = −2; the second, in φ = 120°, to the
plane −(√3/2)x − y/2 = 2. The bottom and the
two cones run on to meet them, so each of them now spans, at distance ρ from the axis, the angle
from −asin(2/ρ) to 120° + asin(2/ρ), and each moved flat is bounded by two hyperbola-like curves
where the planes meet the cones and the bottom; the two planes meet at a line inside the hole
(distance 4/√3 from the axis), clear of the solid.

**As a set:**

$$\{\, (\rho,\ z) \in T,\ y \ge -2,\ -\tfrac{\sqrt{3}}{2}x - \tfrac{y}{2} \le 2 \,\},\quad T = \text{the triangle } (4, 0),\ (10, 0),\ (7, 4)$$
