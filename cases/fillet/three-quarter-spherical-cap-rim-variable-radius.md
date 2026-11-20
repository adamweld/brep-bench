+++
title = "Three quarters of a spherical cap, its flat rim rounded with a radius growing along it"
tags = ["fillet", "variable-radius", "sphere", "revolve", "planar", "analytic"]
origin = "OCCT tests/blend/buildevol/B5, OCCT tests/blend/buildevol/B6, OCCT tests/blend/buildevol/B7, OCCT tests/blend/buildevol/B8, OCCT tests/blend/buildevol/B9, OCCT tests/blend/buildevol/C1"

[volume]
value = 252.83055261365912
exact = "104π − ∫₀^{3π/2} M dφ, r = 1 + φ/(3π), M = the moment ∬ρ dA, in a plane through the axis, of the corner between the flat, the sphere and the round (the region between the sphere ρ² + z² = 100, the flat z = 6 and the round's arc, by Green's theorem ∮ρ²/2 dz), the circle's center at (ρ_c, 6 + r), ρ_c = √(64 − 32r)"
tex = '104\pi - \int_0^{3\pi/2} M\,d\varphi,\quad M = \iint \rho\,dA \text{ of the corner between flat, sphere and round},\ r = 1 + \tfrac{\varphi}{3\pi}'

[area]
value = 262.85015808583603
exact = "108π (three quarters of the flat, 48π, and of the dome, 60π) − ∫₀^{3π/2}(16r + 100((6 + r)/(10 − r) − 3/5)) dφ + A_round + Σ over the two end faces (P − a), P = ∫₀⁸(√(100 − ρ²) − 6)dρ the cap's section through its axis, a = the corner's area in it; A_round = ∫∫|P_φ × P_ψ|dψdφ, P(φ,ψ) = ((ρ_c + r cos ψ)cos φ, (ρ_c + r cos ψ)sin φ, 6 + r + r sin ψ), ψ ∈ [−π/2, atan2(6 + r, ρ_c)], r = 1 + φ/(3π)"
tex = '\begin{array}{l} 108\pi - \displaystyle\int_0^{3\pi/2}\Bigl(16r + 100\bigl(\tfrac{6 + r}{10 - r} - \tfrac35\bigr)\Bigr)d\varphi + \int_0^{3\pi/2}\!\!\int_{-\pi/2}^{\psi_T}\lvert P_\varphi \times P_\psi\rvert\,d\psi\,d\varphi + \sum_{\varphi = 0,\ 3\pi/2}(P - a) \\ P = \bigl((\rho_c + r\cos\psi)\cos\varphi,\ (\rho_c + r\cos\psi)\sin\varphi,\ 6 + r + r\sin\psi\bigr),\ \psi_T = \operatorname{atan2}(6 + r, \rho_c),\ \rho_c = \sqrt{64 - 32r} \end{array}'
+++

Three quarters of a cap of a ball: the part of the ball of radius 10 about the origin with z ≥ 6
(a dome 4 high on a flat circular face of radius 8 in the plane z = 6), turned from φ = 0° to
φ = 270° about the z axis, φ measured from +x toward +y. It has a flat face, three quarters of a disc
of radius 8; a domed face, three quarters of the sphere; and two flat end faces, in the half-planes
φ = 0° and φ = 270°, each the cap's section through its axis, half of it.

Round the one edge where the flat face meets the sphere, the three-quarter circle of radius 8 at
z = 6, with a radius that grows along it: r = 1 + φ/(3π), so 1 where the edge meets the end face at
φ = 0° and 3/2 where it meets the end face at φ = 270°, growing evenly with the angle. The other
edges stay sharp.

The round's sections square to the edge are circular: in the plane through the z axis at each angle
φ, the round's section is an arc of the circle of radius r(φ) that touches the flat face and the
sphere from inside. Its center is at (ρ, z) = (√(64 − 32r), 6 + r), r above the flat and 10 − r from
the ball's center. It touches the flat at (√(64 − 32r), 6) and the sphere at the point where the ray
from the ball's center through the circle's center reaches the sphere. The edge's angle between the
flat and the sphere is 53.13°. The round runs the whole length of the edge and ends in the two end
faces, each of which it meets in such an arc, of radius 1 at φ = 0° and 3/2 at φ = 270°.

**As a set:** with r = 1 + φ/(3π) and (ρ_c, z_c) = (√(64 − 32r), 6 + r),

$$\{\, \lVert p \rVert \le 10,\ z \ge 6,\ 0 \le \varphi \le 3\pi/2 \,\} \setminus \{\, \rho \ge \rho_c,\ (z_c - z) + (\rho - \rho_c) z_c/\rho_c \ge 0,\ (\rho - \rho_c)^2 + (z - z_c)^2 \ge r^2 \,\}$$
