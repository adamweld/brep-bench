+++
title = "A quarter cylinder's curved rim rounded with a radius growing around it"
tags = ["fillet", "variable-radius", "cylinder", "analytic"]
origin = "OCCT tests/blend/buildevol/A4"

[volume]
value = 3080.9270002792849
exact = "1000π − ∫₀^{π/2} (20(1 − π/4)r² − (5/6 − π/4)r³) dφ, r = 2 + 4φ/π: = 1000π − (280π/3)(1 − π/4) + 15π(5/6 − π/4) = 5515π/6 + 235π²/12. In each plane through the axis the round takes the corner of side r less the quarter disk, area (1 − π/4)r², whose moment about the corner along ρ is (5/6 − π/4)r³; turned, it takes ∫∫ρ dA dφ"
tex = '1000\pi - \int_0^{\pi/2}\Bigl(20\bigl(1 - \tfrac{\pi}{4}\bigr)r^2 - \bigl(\tfrac56 - \tfrac{\pi}{4}\bigr)r^3\Bigr)d\varphi = \tfrac{5515\pi}{6} + \tfrac{235\pi^2}{12},\quad r = 2 + \tfrac{4\varphi}{\pi}'

[area]
value = 1296.7248425789726
exact = "300π + 400 (the quarter cylinder) − 83π/3 (the top's band under the round, ∫ (20² − (20 − r)²)/2 dφ) − 30π (the side's band, ∫ 20r dφ) − (1 − π/4)(2² + 4²) (the two flats' corners) + I = 380 + 742π/3 + I, I = ∫₀^{π/2}∫₀^{π/2} r√((20 − r + r cos ψ)² + (4/π)²(1 − cos ψ − sin ψ)²) dψ dφ = 139.70425959109706276, the round's area"
tex = '\begin{array}{l} 380 + \tfrac{742\pi}{3} + I \\ I = \displaystyle\int_0^{\pi/2}\!\!\int_0^{\pi/2} r\sqrt{(20 - r + r\cos\psi)^2 + \tfrac{16}{\pi^2}(1 - \cos\psi - \sin\psi)^2}\,d\psi\,d\varphi = 139.70425959109706 \end{array}'
+++

A quarter of a cylinder, its curved top rim rounded with a radius that grows from one end of the
rim to the other.

The quarter cylinder: the part of the cylinder of radius 20 on the z axis, from z = 0 to z = 10,
with x ≥ 0 and y ≥ 0. It has two flat sides, in the planes y = 0 and x = 0, a quarter of a
cylindrical side, and quarter-disk top and bottom.

Round the edge where the top (z = 10) meets the curved side (ρ = 20): the quarter circle from
(20, 0, 10) to (0, 20, 10). At the angle φ about the z axis, measured from +x toward +y, the
radius is r = 2 + 4φ/π: 2 where the edge meets the flat side y = 0 (φ = 0) and 4 where it meets
the flat side x = 0 (φ = 90°), growing evenly with the angle between.

The round's sections square to the edge are circular: in the plane through the z axis at each
angle φ, the round's section is the quarter circle of radius r(φ) centred at (ρ, z) = (20 − r,
10 − r), tangent to the top at ρ = 20 − r and to the side at z = 10 − r. The round meets each flat
side in such a quarter circle, of radius 2 on y = 0 and 4 on x = 0.

**As a set:** with r = 2 + 4φ/π,

$$\{\, x, y \ge 0,\ \rho \le 20,\ 0 \le z \le 10 \,\} \setminus \{\, \rho > 20 - r,\ z > 10 - r,\ (\rho - 20 + r)^2 + (z - 10 + r)^2 > r^2 \,\}$$
