+++
title = "A turned shoulder's inside corner rounded with a radius growing round a quarter turn"
tags = ["fillet", "variable-radius", "concave", "cylinder", "revolve", "analytic"]
origin = "OCCT tests/blend/buildevol/D7, OCCT tests/blend/buildevol/D8"

[volume]
value = 1186.1539508123740
exact = "375π + ∫₀^{π/2} M dφ, M = (5 − 5π/4)r² + (5/6 − π/4)r³ the moment ∬ρ dA, in a plane through the axis, of the square of side r in the corner less its quarter disc (which is added), r = 1 + 4φ/π: ∫r² dφ = 13π/6, ∫r³ dφ = 5π, so 390π − 95π²/24"
tex = '375\pi + \int_0^{\pi/2}\Bigl(\bigl(5 - \tfrac{5\pi}{4}\bigr)r^2 + \bigl(\tfrac56 - \tfrac{\pi}{4}\bigr)r^3\Bigr)d\varphi = 390\pi - \tfrac{95\pi^2}{24},\quad r = 1 + \tfrac{4\varphi}{\pi}'

[area]
value = 867.18849060441453
exact = "A₀ − ∫₀^{π/2}(10r + r²/2)dφ + A_round + 10(1 − π/4), A₀ = 150π + 400 (the quarter without the round: its two curved sides 50π and 50π, the discs 25π and 25π/4, the shoulder 75π/4, the two flat cuts 2·200); the round covers the shoulder band ((5 + r)² − 25)/2 and the wall band 5r per radian and adds the two cuts the corner’s area (1 − π/4)r², at r = 1 and r = 3; A_round = ∫∫|P_φ × P_ψ|dψdφ, P(φ,ψ) = ((5 + r + r cos ψ)cos φ, (5 + r + r cos ψ)sin φ, 10 + r + r sin ψ), ψ ∈ [π, 3π/2], r = 1 + 4φ/π"
tex = 'A_0 - \int_0^{\pi/2}\bigl(10r + \tfrac{r^2}{2}\bigr)d\varphi + A_{\mathrm{round}} + 10\bigl(1 - \tfrac{\pi}{4}\bigr),\quad A_0 = 150\pi + 400,\quad A_{\mathrm{round}} = \displaystyle\int_0^{\pi/2}\!\!\int_{\pi}^{3\pi/2}\lvert P_\varphi \times P_\psi\rvert\,d\psi\,d\varphi'
+++

A quarter of a shaft with a shoulder, the part with x ≥ 0 and y ≥ 0: a cylinder of radius 10 on the z
axis from z = 0 to z = 10, and on it a cylinder of radius 5 from z = 10 to z = 30. The big
cylinder's top, the ring from ρ = 5 to 10 at z = 10, is the shoulder. Round the inside corner where
the shoulder meets the small cylinder, the quarter circle of radius 5 at z = 10 from (5, 0, 10) to
(0, 5, 10), with a radius that grows evenly with the angle φ about the z axis, measured from +x
toward +y: r = 1 + 4φ/π, so 1 where the edge meets the flat side y = 0 and 3 where it meets the flat
side x = 0. The round adds material: it fills the corner.

The round's sections square to the edge are circular: in the plane through the z axis at each angle
φ, the round's section is the quarter circle of radius r(φ) centered at (ρ, z) = (5 + r, 10 + r),
tangent to the shoulder at ρ = 5 + r and to the small cylinder at z = 10 + r; the round fills the
square corner between them out to that quarter circle. The round meets each flat side in such a
quarter circle, of radius 1 on y = 0 and 3 on x = 0.

**As a set:** with r = 1 + 4φ/π, over 0 ≤ φ ≤ π/2,

$$\{\,\rho \le 10,\ 0 \le z \le 10\,\} \cup \{\,\rho \le 5,\ 10 \le z \le 30\,\} \cup \{\,5 \le \rho \le 5 + r,\ 10 \le z \le 10 + r,\ (\rho - 5 - r)^2 + (z - 10 - r)^2 \ge r^2\,\}$$
