+++
title = "A quarter of a cone frustum, its top rim rounded with a radius growing along it"
tags = ["fillet", "variable-radius", "cone", "revolve", "planar", "analytic"]
origin = "OCCT tests/blend/buildevol/A8, OCCT tests/blend/buildevol/A9, OCCT tests/blend/buildevol/B1, OCCT tests/blend/buildevol/B2, OCCT tests/blend/buildevol/B3, OCCT tests/blend/buildevol/B4"

[volume]
value = 612.18421454850391
exact = "196π − ∫₀^{π/2} M dφ, r = 3/2 + 2φ/π, M = the moment ∬ρ dA, in a plane through the axis, of the corner between the top, the cone and the round: the kite on the rim (tangent length t = r(√10 − 1)/3 on both sides), less the circle's sector, by Green's theorem ∮ρ²/2 dz"
tex = '196\pi - \int_0^{\pi/2} M\,d\varphi,\quad M = \iint \rho\,dA \text{ of the corner kite less the circle sector},\ r = \tfrac32 + \tfrac{2\varphi}{\pi}'

[area]
value = 453.07615776210729
exact = "25π + 9π + 4π√160 (a quarter of the bottom, the top and the cone) − ∫₀^{π/2}(B_top + B_cone) dφ + A_round + Σ over the two end faces (96 − (r t − r²(π − ω)/2)), cos ω = −1/√10, t = r(√10 − 1)/3, r = 3/2 + 2φ/π; B_top = (36 − ρ_c²)/2, B_cone = 6t + t²/(2√10), ρ_c = 6 − t; A_round = ∫∫|P_φ × P_ψ|dψdφ, P(φ,ψ) = ((ρ_c + r cos ψ)cos φ, (ρ_c + r cos ψ)sin φ, 12 − r + r sin ψ), ψ ∈ [atan(1/3), π/2]"
tex = '\begin{array}{l} 34\pi + 4\pi\sqrt{160} - \displaystyle\int_0^{\pi/2}\!\bigl(B_{top} + B_{cone}\bigr)d\varphi + \int_0^{\pi/2}\!\!\int_{\arctan(1/3)}^{\pi/2}\lvert P_\varphi \times P_\psi\rvert\,d\psi\,d\varphi + \sum_{\varphi = 0,\ \pi/2}\bigl(96 - rt + \tfrac{r^2(\pi - \omega)}{2}\bigr) \\ P = \bigl((\rho_c + r\cos\psi)\cos\varphi,\ (\rho_c + r\cos\psi)\sin\varphi,\ 12 - r + r\sin\psi\bigr),\ \rho_c = 6 - \tfrac{(\sqrt{10} - 1)r}{3} \end{array}'
+++

A quarter of a cone frustum: the part with x ≥ 0 and y ≥ 0 of the frustum on the z axis with radius 10
at its bottom, z = 0, and radius 6 at its top, z = 12. It has a quarter-disc bottom and top, a quarter
of the cone for a side, and two flat end faces, one in the plane y = 0 and one in the plane x = 0,
each a trapezoid (the frustum's section through its axis, half of it).

Round the one edge where the top meets the cone, the quarter circle of radius 6 at z = 12 from
(6, 0, 12) to (0, 6, 12), with a radius that grows along it: at the angle φ about the z axis, measured
from +x toward +y, r = 3/2 + 2φ/π. So the radius is 3/2 where the edge meets the end face y = 0
(φ = 0) and 5/2 where it meets the end face x = 0 (φ = 90°), growing evenly with the angle. The other
edges stay sharp.

The round's sections square to the edge are circular: in the plane through the z axis at each angle
φ, the round's section is an arc of the circle of radius r(φ) that touches the top and the cone from
inside, its center at (ρ, z) = (6 − r(√10 − 1)/3, 12 − r). The cone is the line 3ρ + z = 30 in that
plane; the circle touches the top at (6 − r(√10 − 1)/3, 12) and the cone at the center plus
r(3, 1)/√10. The corner of the top and the cone, an angle of about 108.4° there, is cut off by the
arc between them. The round runs the whole length of the edge and ends in the two end faces, each of
which it meets in such an arc, of radius 3/2 on y = 0 and 5/2 on x = 0.

**As a set:** with r = 3/2 + 2φ/π and ρ_c = 6 − r(√10 − 1)/3,

$$\{\, x, y \ge 0,\ 0 \le z \le 12,\ 3\rho + z \le 30 \,\} \setminus \{\, \rho \ge \rho_c,\ z - 12 + r \ge (\rho - \rho_c)/3,\ (\rho - \rho_c)^2 + (z - 12 + r)^2 \ge r^2 \,\}$$
