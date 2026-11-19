+++
title = "A drum standing in a torus, its side drafted"
tags = ["draft", "cone", "torus", "boolean", "analytic"]
origin = "OCCT tests/draft/angle/G1"

[volume]
value = 13204.343265250613
exact = "2π(M₁ + M₂), by Pappus, the section's moment about the axis by Green's theorem round its boundary: the tube's arc of π + ψ radians outside the drum, M₁ = (r/2)(R²s + Rr(π + ψ + sin 2ψ/2) + r²(s − s³/3)), and the drafted side, M₂ = (12 − z_a)(ρ_a² + ρ_aρ₁ + ρ₁²)/6; R = 15, r = 5, T = tan 5°, z_a = (−T + √(25(1 + T²) − 1))/(1 + T²), ρ_a = 14 − z_aT, ρ₁ = 14 − 12T, s = z_a/r, ψ = π − asin s"
tex = '2\pi\left(M_1 + M_2\right),\quad M_1 = \tfrac{r}{2}\left(R^2 s + Rr\left(\pi + \psi + \tfrac{\sin 2\psi}{2}\right) + r^2\left(s - \tfrac{s^3}{3}\right)\right),\ M_2 = \tfrac{(12 - z_a)\left(\rho_a^2 + \rho_a\rho_1 + \rho_1^2\right)}{6},\quad z_a = \tfrac{-T + \sqrt{25(1+T^2) - 1}}{1 + T^2},\ \rho_a = 14 - z_aT,\ \rho_1 = 14 - 12T,\ s = \tfrac{z_a}{r},\ \psi = \pi - \arcsin s,\ R = 15,\ r = 5,\ T = \tan 5^\circ'

[area]
value = 3950.8267298530495
exact = "2π((R − r)²/2 + rR(π + ψ) + r·z_a + (ρ_a + ρ₁)(12 − z_a)/(2 cos 5°) + ρ₁²/2): by Pappus, the drum's bottom inside the hole, the torus outside the drum, the drafted side and the top"
tex = '2\pi\left(\tfrac{(R - r)^2}{2} + rR(\pi + \psi) + r z_a + \frac{(\rho_a + \rho_1)(12 - z_a)}{2\cos 5^\circ} + \tfrac{\rho_1^2}{2}\right)'
+++

A ring torus about the z axis, the solid swept by the disk of radius 5 centered on (15, 0, 0) in
the plane y = 0 turned a full turn about the z axis, and a drum, the cylinder of radius 14 about the
z axis from z = 0 to z = 12, united. The drum's side runs down into the tube, and its bottom, in
the plane z = 0, shows only inside the torus's hole, as the disk of radius 10.

Draft the drum's side by 5°, with the pull direction +z and the neutral plane z = 0: the side
becomes the cone through the drum's circle of radius 14 at z = 0, leaning 5° from the z direction,
inward as it rises, its radius 14 − z·tan 5°. It meets the torus at z_a ≈ 4.794, radius
ρ_a ≈ 13.581, and the top shrinks to meet it, to radius 14 − 12·tan 5° ≈ 12.950. The torus and the
drum's top and bottom are not drafted.

So the result is the torus with a frustum of the cone standing in it.

**As a set:**

$$\{\, (\rho - 15)^2 + z^2 \le 25 \,\} \cup \{\, \rho \le 14 - z\tan 5^\circ,\ 0 \le z \le 12 \,\}$$
