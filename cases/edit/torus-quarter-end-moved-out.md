+++
title = "A quarter of a torus, one end moved out"
tags = ["edit", "move-face", "torus", "analytic"]
origin = "OCCT tests/offset/faces_type_i/D8"

[volume]
value = 501.09424015363055
exact = "∫ from R − r to R + r of 2√(r² − (ρ − R)²)·ρ·(π/2 + asin(d/ρ)) dρ, R = 10, r = 3, d = 2: each ring of the tube at radius ρ, of height 2√(r² − (ρ − R)²), spans the angles −asin(d/ρ) to 90° about the axis"
tex = '\int_{R-r}^{R+r} 2\sqrt{r^2 - (\rho - R)^2}\,\rho\left(\tfrac{\pi}{2} + \arcsin\tfrac{d}{\rho}\right)d\rho,\quad R = 10,\ r = 3,\ d = 2'

[area]
value = 391.26206259719635
exact = "∫ from 0 to 2π of r·ρ·(π/2 + asin(d/ρ)) dθ + πr² + A_d, ρ = R + r·cos θ, R = 10, r = 3, d = 2: the torus's piece (area element rρ dθ dφ), the end in x = 0, a disk, and the moved end in y = −2, whose area is A_d = ∫ from √((R − r)² − d²) to √((R + r)² − d²) of 2√(r² − (√(x² + d²) − R)²) dx = 28.9035…, the area of the section of the tube by the plane y = d, x ≥ 0 (and, by symmetry, by x = d), taken along x"
tex = '\int_0^{2\pi} r\rho\left(\tfrac{\pi}{2} + \arcsin\tfrac{d}{\rho}\right)d\theta + \pi r^2 + A_d,\quad \rho = R + r\cos\theta,\ A_d = \int_{\sqrt{(R-r)^2 - d^2}}^{\sqrt{(R+r)^2 - d^2}} 2\sqrt{r^2 - \left(\sqrt{x^2 + d^2} - R\right)^2}\,dx'
+++

A quarter of the solid torus about the z axis whose tube, of radius 3, runs round the circle of
radius 10 in the plane z = 0: the part whose angle about the z axis, φ, measured from +x toward +y,
lies between 0° and 90°. Its faces are the torus and two flat disks of radius 3, its ends, in the
half-planes y = 0, x ≥ 0 (φ = 0°) and x = 0, y ≥ 0 (φ = 90°).

Then edit the solid directly: move the end at φ = 0° by 2 outward along its normal, −y, to the plane
y = −2. The torus runs on as the same surface into the part before φ = 0°, as far as that plane,
which cuts it in a flat face that is no longer a disk: the section of the tube by y = −2, with x ≥
0. The other end is untouched, a quarter of a turn away.

So the result is the quarter and the part of the torus with x ≥ 0 and −2 ≤ y ≤ 0.

**As a set:**

$$\{\, (\rho - 10)^2 + z^2 \le 9,\ x \ge 0,\ y \ge -2 \,\}$$
