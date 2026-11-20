+++
title = "Three quarters of a torus, both its ends moved out"
tags = ["edit", "move-face", "torus", "analytic"]
origin = "OCCT tests/offset/faces_type_i/E5"

[volume]
value = 1446.3206783562823
exact = "2π²Rr² − ∫ from R − r to R + r of 2√(r² − (ρ − R)²)·ρ·(π/2 − 2·asin(d/ρ)) dρ, R = 10, r = 3, d = 2: the torus less its part with x > d and y < −d, each ring of which spans the angles −acos(d/ρ) to −asin(d/ρ)"
tex = '2\pi^2 R r^2 - \int_{R-r}^{R+r} 2\sqrt{r^2 - (\rho - R)^2}\,\rho\left(\tfrac{\pi}{2} - 2\arcsin\tfrac{d}{\rho}\right)d\rho,\quad R = 10,\ r = 3,\ d = 2'

[area]
value = 1022.0635894624572
exact = "4π²Rr − ∫ from 0 to 2π of r·ρ·(π/2 − 2·asin(d/ρ)) dθ + 2A_d, ρ = R + r·cos θ, R = 10, r = 3, d = 2: the whole torus less the piece with x > d and y < −d, and two moved ends, each of area A_d = ∫ from √((R − r)² − d²) to √((R + r)² − d²) of 2√(r² − (√(x² + d²) − R)²) dx = 28.9035…, the area of the section of the tube by the plane y = d, x ≥ 0 (and, by symmetry, by x = d), taken along x"
tex = '4\pi^2 R r - \int_0^{2\pi} r\rho\left(\tfrac{\pi}{2} - 2\arcsin\tfrac{d}{\rho}\right)d\theta + 2A_d,\quad \rho = R + r\cos\theta,\ A_d = \int_{\sqrt{(R-r)^2 - d^2}}^{\sqrt{(R+r)^2 - d^2}} 2\sqrt{r^2 - \left(\sqrt{x^2 + d^2} - R\right)^2}\,dx'
+++

Three quarters of the solid torus about the z axis whose tube, of radius 3, runs round the circle of
radius 10 in the plane z = 0: the part whose angle about the z axis, φ, measured from +x toward +y,
lies between 0° and 270°. Its faces are the torus and two flat disks of radius 3, its ends, in the
half-planes y = 0, x ≥ 0 (φ = 0°) and x = 0, y ≤ 0 (φ = 270°).

Then edit the solid directly: move the end at φ = 0° by 2 outward along its normal, −y, to the plane
y = −2, and the end at φ = 270° by 2 outward along its normal, +x, to the plane x = 2. The torus
runs on round into the quarter that was missing, as far as them: each end is now the section of the
tube by that plane, no longer a disk, y = −2 with x ≥ 2 and x = 2 with y ≤ −2. The two ends do not
meet.

So the result is the torus less its part with x > 2 and y < −2.

**As a set:**

$$\{\, (\rho - 10)^2 + z^2 \le 9 \,\} \setminus \{\, x > 2,\ y < -2 \,\}$$
