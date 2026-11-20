+++
title = "A quarter of a cone frustum, one flat moved out"
tags = ["edit", "move-face", "cone", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/I8, I9, J1, J2"

[volume]
value = 597.7887963316075
exact = "356π/3 + 20√91 − 36√3 − 9·ln((10 + √91)/(6 + 3√3)) + (1000/3)·asin(3/10): V = 2∫ A(ρ) dρ from ρ = 6 to 10, where the cross-section at radius ρ has area A = πρ²/4 + (d√(ρ² − d²) + ρ²·asin(d/ρ))/2, d = 3 (the quarter disk plus the segment between y = −3 and y = 0)"
tex = '\tfrac{356\pi}{3} + 20\sqrt{91} - 36\sqrt{3} - 9\ln\tfrac{10+\sqrt{91}}{6+3\sqrt{3}} + \tfrac{1000}{3}\arcsin\tfrac{3}{10}'

[area]
value = 440.7380563222488
exact = "37π + 88 + (23/2)√91 − (27/2)√3 − 9·ln((10 + √91)/(6 + 3√3)) + 50·asin(3/10) + √5(13π + 50·asin(3/10) + (3/2)√91 − (9/2)√3): the wall, √5·∫ρ(π/2 + asin(d/ρ)) dρ from 6 to 10; the flat x = 0, a trapezoid 2∫(ρ + d) dρ; the new flat y = −3, 2∫√(ρ² − d²) dρ; and the bottom and top, A(10) + A(6)"
tex = '37\pi + 88 + \tfrac{23}{2}\sqrt{91} - \tfrac{27}{2}\sqrt{3} - 9\ln\tfrac{10+\sqrt{91}}{6+3\sqrt{3}} + 50\arcsin\tfrac{3}{10} + \sqrt{5}\left(13\pi + 50\arcsin\tfrac{3}{10} + \tfrac{3}{2}\sqrt{91} - \tfrac{9}{2}\sqrt{3}\right)'
+++

A quarter of a cone frustum about the z axis, of radius 10 at z = 0 and 6 at z = 8: the part whose angle about the z axis, φ, measured from +x toward +y, lies between 0° and 90°, so the part with x ≥ 0 and y ≥ 0. Its faces are the cone, a quarter of it; a bottom and a top, quarter disks of radius 10 and 6; and two flat trapezoids, in the half-planes y = 0, x ≥ 0 and x = 0, y ≥ 0, which meet along the z axis in a convex edge.

Then edit the solid directly: move the flat in the plane y = 0 out by 3 along its outward normal, −y, to the plane y = −3. The cone runs on round past the old edge to meet it, and the bottom and the top grow to follow; the other flat is lengthened from y = 0 to y = −3.

So the result is the frustum cut to x ≥ 0 and y ≥ −3.

**As a set:**

$$\{\, \rho \le 10 - \tfrac{z}{2},\ 0 \le z \le 8,\ x \ge 0,\ y \ge -3 \,\}$$
