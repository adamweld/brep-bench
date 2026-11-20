+++
title = "A quarter of a triangular ring, both ends moved in"
tags = ["edit", "move-face", "cone", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/M7, M9"

[volume]
value = 8.521981711106197
exact = "∫₃⁵ h(ρ)·ρ·(π/2 − 2·asin(1/ρ)) dρ, h(ρ) = 2 − 2|ρ − 4| the height of the section at radius ρ, the angle at radius ρ running from asin(1/ρ) to π/2 − asin(1/ρ). It is 8π less the volume of `edit/triangle-ring-ends-moved-out` (the two together are the quarter ring's volume 4π twice over), in closed form 4π − (250/3)asin(1/5) + (256/3)asin(1/4) − 18·asin(1/3) − 8√2 − (40/3)√6 + (32/3)√15 − (4/3)ln(4 + √15) + (2/3)ln(3 + 2√2) + (2/3)ln(5 + 2√6)"
tex = '\int_3^5 h(\rho)\,\rho\left(\tfrac{\pi}{2} - 2\arcsin\tfrac{1}{\rho}\right)d\rho,\quad h(\rho) = 2 - 2\,\lvert\rho - 4\rvert'

[area]
value = 31.708616076313696
exact = "(1 + √5)·P + 2·E: P = 4π − 25·asin(1/5) + 9·asin(1/3) − 2√6 + 2√2 = ∫₃⁵ ρ(π/2 − 2·asin(1/ρ)) dρ, the area the solid covers on the plane z = 0, which is the base's area and 1/√5 of the cones' area (both cones rise at a slope of 2); E = 6√2 + 10√6 − 8√15 + 2·ln(4 + √15) − ln(3 + 2√2) − ln(5 + 2√6) = ∫ h(√(x² + 1)) dx from √8 to √24, the area of each moved end (a hyperbolic section of the cones, in the plane y = 1 or x = 1)"
tex = '(1 + \sqrt5)\,P + 2E,\quad P = \int_3^5 \rho\left(\tfrac{\pi}{2} - 2\arcsin\tfrac1\rho\right)d\rho,\ E = \int_{\sqrt8}^{\sqrt{24}} h\bigl(\sqrt{x^2 + 1}\bigr)\,dx'
+++

A piece of a ring with a triangular section: the triangle in the
half-plane y = 0, x > 0 with corners at (x, z) = (3, 0), (5, 0) and (4, 2), turned about the z axis
through a quarter turn, from y = 0 toward x = 0 (φ from 0° to 90°, measured from +x toward +y). Its
faces are the base, a quarter of an annulus from radius 3 to radius 5 in the plane z = 0; two cones
about the z axis, the inner one rising 2 over a radius of 1 from radius 3 at z = 0 to radius 4 at
z = 2, and the outer one falling back to radius 5 at z = 0; and two flat triangles, the ends, in
the planes y = 0 (x ≥ 0) and x = 0 (y ≥ 0).

Then edit the solid directly: move both ends in by 1 along their outward normals, which point
into the solid, the one in the plane y = 0 to y = 1 and the one in x = 0 to x = 1. The cones and
the base are trimmed to meet them, and each moved end is the part of its plane inside the ring,
bounded by hyperbolas where the cones cut it.

**As a set:**

$$\{\,0 \le z \le 2 - 2\lvert\rho - 4\rvert,\ x \ge 1,\ y \ge 1\,\}$$
