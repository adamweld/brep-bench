+++
title = "A quarter of a cone, both its flats moved out past its axis"
tags = ["edit", "move-face", "cone", "apex", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/A9, OCCT tests/offset/faces_type_i/B2"

[volume]
value = 532.36895328441319
exact = "πR²H/3 − (2H/R)·S + (H/R)·T, R = 8, H = 12, d = 3, w = √(R² − d²) = √55: the cone less its parts beyond the planes x = −d and y = −d, each (H/R)·S with S = R³·acos(d/R)/3 − 2dRw/3 + d³·ln((R + w)/d)/3 (the integral over the cone's slices, of radius s, of the segment s²·acos(d/s) − d√(s² − d²)), plus the part beyond both, which they share, (H/R)·T with T = ∫ from d√2 to R of (s²(π/4 − asin(d/s)) − d√(s² − d²) + d²) ds"
tex = '\tfrac{\pi R^2 H}{3} - \tfrac{2H}{R}S + \tfrac{H}{R}T,\quad S = \tfrac{R^3}{3}\arccos\tfrac{d}{R} - \tfrac{2}{3}dRw + \tfrac{d^3}{3}\ln\tfrac{R+w}{d},\quad T = \int_{d\sqrt2}^{R}\left(s^2\left(\tfrac{\pi}{4} - \arcsin\tfrac{d}{s}\right) - d\sqrt{s^2 - d^2} + d^2\right)ds'

[area]
value = 405.32492560728587
exact = "(1 + L/R)·Ω + 2F, L = √(R² + H²) = 4√13, Ω = πR² − 2(R²·acos(d/R) − dw) + (R²(π/4 − asin(d/R)) − dw + d²), F = H(w + d) − (H/R)(wR/2 + (d²/2)·ln((R + w)/d) + (d²/2)(√2 + ln(1 + √2))): the cone's side (L/R times the area beneath it) and its base, both over the footprint Ω, the cone's disk less the segments beyond x = −d and y = −d plus the corner they share; and the two moved flats, each ∫ from x = −d to w of the cone's height H(1 − √(x² + d²)/R)"
tex = '\left(1 + \tfrac{L}{R}\right)\Omega + 2F,\quad L = \sqrt{R^2 + H^2},\ \Omega = \pi R^2 - 2\left(R^2\arccos\tfrac{d}{R} - dw\right) + \left(R^2\left(\tfrac{\pi}{4} - \arcsin\tfrac{d}{R}\right) - dw + d^2\right)'
+++

A quarter of the cone whose base, of radius 8, lies in the plane z = 0 centered on the origin and
whose apex is (0, 0, 12): the part whose angle about the z axis, φ, measured from +x toward +y, lies
between 0° and 90°, so the part with x ≥ 0 and y ≥ 0. Its faces are the cone; the base, a quarter
disk; and two flat triangles through the axis, in the half-plane y = 0, x ≥ 0 and in the half-plane
x = 0, y ≥ 0, which meet along the axis, from the base's center to the apex, in a convex edge.

Then edit the solid directly: move each flat 3 outward along its normal, the first to the plane
y = −3 and the second to the plane x = −3. The cone and the base run on round past the old edge to
meet both moved flats, and the two flats are extended until they meet each other, along the line
x = y = −3 from the base up to the cone at z = 12(1 − 3√2/8) ≈ 5.64. The apex, where the old edge
met the cone, is left inside the conical face.

So the result is the cone less everything at x < −3 or y < −3.

**As a set:**

$$\{\, \rho \le 8\left(1 - \tfrac{z}{12}\right),\ 0 \le z \le 12 \,\} \cap \{\, x \ge -3,\ y \ge -3 \,\}$$
