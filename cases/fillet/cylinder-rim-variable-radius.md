+++
title = "A cylinder's top rim rounded with a radius that varies all the way round it"
tags = ["fillet", "variable-radius", "cylinder", "closed-edge", "analytic"]
origin = "OCCT tests/blend/buildevol/E4"

[volume]
value = 6164.5762570455770
exact = "2000π − ∫₀^{2π} M dφ, M = (10 − 5π/2)r² − (5/6 − π/4)r³ the moment ∬ρ dA of the corner of side r less its quarter disc in a plane through the axis, r = 3 − cos φ: ∫r² dφ = 19π, ∫r³ dφ = 63π, so 2000π − 275π/2 + 127π²/4"
tex = '2000\pi - \int_0^{2\pi}\Bigl(\bigl(10 - \tfrac{5\pi}{2}\bigr)r^2 - \bigl(\tfrac56 - \tfrac{\pi}{4}\bigr)r^3\Bigr)d\varphi = 2000\pi - \tfrac{275\pi}{2} + \tfrac{127\pi^2}{4},\quad r = 3 - \cos\varphi'

[area]
value = 1799.9012560475708
exact = "600π − ∫₀^{2π}(20r − r²/2)dφ + A_round = 489.5π + A_round (the top loses the band (100 − (10 − r)²)/2 per radian, the side 10r) where A_round = ∫∫|P_φ × P_ψ|dψdφ = 262.1239…, P(φ,ψ) = ((10 − r + r cos ψ)cos φ, (10 − r + r cos ψ)sin φ, 20 − r + r sin ψ), ψ ∈ [0, π/2], r = 3 − cos φ"
tex = '\begin{array}{l} 489.5\pi + A_{\mathrm{round}},\quad A_{\mathrm{round}} = \displaystyle\int_0^{2\pi}\!\!\int_0^{\pi/2}\lvert P_\varphi \times P_\psi\rvert\,d\psi\,d\varphi \\ P = \bigl((10 - r + r\cos\psi)\cos\varphi,\ (10 - r + r\cos\psi)\sin\varphi,\ 20 - r + r\sin\psi\bigr),\ r = 3 - \cos\varphi \end{array}'
+++

A cylinder of radius 10 on the z axis, from z = 0 to z = 20. Round the circular edge around its top
face with a radius that varies all the way round it: at the angle φ about the z axis, measured from
+x toward +y, r = 3 − cos φ. So the radius is 2 at φ = 0, where the edge's seam is, 4 opposite it at
φ = 180°, and varies smoothly between, with no kink where it closes.

The round's sections square to the edge are circular: in the plane through the z axis at each angle
φ, the round's section is the quarter circle of radius r(φ) centered at (ρ, z) = (10 − r, 20 − r),
tangent to the top at ρ = 10 − r and to the side at z = 20 − r.

**As a set:** with r = 3 − cos φ,

$$\{\,\rho \le 10,\ 0 \le z \le 20\,\} \setminus \{\,\rho > 10 - r,\ z > 20 - r,\ (\rho - 10 + r)^2 + (z - 20 + r)^2 > r^2\,\}$$
