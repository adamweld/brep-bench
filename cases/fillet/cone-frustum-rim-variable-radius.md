+++
title = "A cone frustum's top rim rounded with a radius that varies all the way round it"
tags = ["fillet", "variable-radius", "cone", "closed-edge", "analytic"]
origin = "OCCT tests/blend/buildevol/E5"

[volume]
value = 2448.6016508199607
exact = "784π − ∫₀^{2π} M dφ, M = the moment ∬ρ dA, in a plane through the axis, of the corner between the top, the cone and the round, r = 2 − (cos φ)/2: the kite on the rim, the two tangent points and the circle’s center (tangent length r/tan(ω/2), cos ω = −1/√10) less the circle’s sector"
tex = '784\pi - \int_0^{2\pi} M\,d\varphi,\quad M = \iint \rho\,dA \text{ of the corner kite less the circle sector},\ r = 2 - \tfrac12\cos\varphi'

[area]
value = 1047.5553754564664
exact = "100π + 36π + 16π√160 − ∫(top and cone bands lost) dφ + A_round, A_round = ∫∫|P_φ × P_ψ|dψdφ, P(φ,ψ) = ((ρ_c + r cos ψ)cos φ, (ρ_c + r cos ψ)sin φ, 12 − r + r sin ψ), ρ_c = 6 − r(√10 − 1)/3, ψ ∈ [atan(1/3), π/2], r = 2 − (cos φ)/2; the bands: the top from ρ_c to 6, and the cone for r/tan(ω/2) along its slant, each ∫ρ ds per radian"
tex = '\begin{array}{l} 136\pi + 16\pi\sqrt{160} - \displaystyle\int_0^{2\pi}\! B(r)\,d\varphi + \int_0^{2\pi}\!\!\int_{\arctan(1/3)}^{\pi/2}\lvert P_\varphi \times P_\psi\rvert\,d\psi\,d\varphi \\ P = \bigl((\rho_c + r\cos\psi)\cos\varphi,\ (\rho_c + r\cos\psi)\sin\varphi,\ 12 - r + r\sin\psi\bigr),\ \rho_c = 6 - \tfrac{(\sqrt{10} - 1)r}{3} \end{array}'
+++

A cone frustum on the z axis: radius 10 at its bottom, z = 0, and radius 6 at its top, z = 12. Round
the circular edge around its top face with a radius that varies all the way round it: at the angle φ
about the z axis, measured from +x toward +y, r = 2 − (cos φ)/2. So the radius is 1.5 at φ = 0, where
the edge's seam is, 2.5 opposite it at φ = 180°, and varies smoothly between.

The round's sections square to the edge are circular: in the plane through the z axis at each angle
φ, the round's section is an arc of the circle of radius r(φ) that touches the top and the cone from
inside, its center at (ρ, z) = (6 − r(√10 − 1)/3, 12 − r). The cone is the line 3ρ + z = 30 in that
plane; the circle touches the top at (6 − r(√10 − 1)/3, 12) and the cone at the center plus
r(3, 1)/√10. The arc between them is the round's section: the corner of the top and the cone, an
angle of about 108.4° there, is cut off by it.
