+++
title = "The apex of a triangular ring rounded with a radius growing round a quarter turn"
tags = ["fillet", "variable-radius", "cone", "revolve", "analytic"]
origin = "OCCT tests/blend/buildevol/D9, OCCT tests/blend/buildevol/E1"

[volume]
value = 1288.9856516391263
exact = "430π − ∫₀^{π/2} M dφ, M = the moment ∬ρ dA, in a plane through the axis, of the corner of the triangle at the apex between its two sides and the round: the kite on the apex, the two tangent points and the circle’s center (tangent length r/tan(ω/2), cos ω = 123/√(153·193)) less the circle’s sector; r = 1 + 2φ/π"
tex = '430\pi - \int_0^{\pi/2} M\,d\varphi,\quad M = \iint\rho\,dA\ \text{of the kite less the sector},\ r = 1 + \tfrac{2\varphi}{\pi}'

[area]
value = 854.31328244826974
exact = "A₀ − ∫₀^{π/2} B(r)dφ + A_round − (region area at r = 1 and at r = 2: the two cuts each lose the corner’s area), A₀ = 75π + (π/2)(11.5)√153 + (π/2)(16.5)√193 + 120 (the bottom ring, the two cones, the two flat cuts of 60 each); B(r) = ∫ρ ds along both sides for the tangent length r/tan(ω/2) from the apex, per radian; A_round = ∫∫|P_φ × P_ψ|dψdφ over the round P(φ,ψ) = (ρ cos φ, ρ sin φ, z), (ρ, z) = (13, 12) + r(κ_ρ(ψ), κ_z(ψ)), κ = b/sin(ω/2) + (cos ψ, sin ψ), b the unit bisector at the apex pointing into the triangle, ψ between the directions from the center to the two tangent points"
tex = 'A_0 - \int_0^{\pi/2}\! B(r)\,d\varphi + A_{\mathrm{round}} - \bigl(\mathrm{corner}(1) + \mathrm{corner}(2)\bigr),\quad A_0 = 75\pi + \tfrac{\pi}{2}\bigl(11.5\sqrt{153} + 16.5\sqrt{193}\bigr) + 120'
+++

A quarter of a ring with a triangular section: the triangle with corners (ρ, z) = (10, 0), (20, 0)
and (13, 12) in the half-plane y = 0, x > 0, turned about the z axis from +x a quarter of the way
round toward +y. It has a flat bottom ring from ρ = 10 to 20, an inner cone from (10, 0) up to the
apex (13, 12), an outer cone from (20, 0) up to it, and two flat triangular ends in the planes
y = 0 and x = 0. Round the apex, the quarter circle of radius 13 at z = 12 where the two cones meet,
with a radius that grows evenly with the angle φ about the z axis, measured from +x toward +y:
r = 1 + 2φ/π, so 1 at the end in y = 0 and 2 at the end in x = 0.

The round's sections square to the edge are circular: in the plane through the z axis at each angle
φ, the round's section is the arc of the circle of radius r(φ) inscribed in the triangle's corner at
the apex, tangent to both its sides inside it. (Its center lies on the bisector of the corner, a
distance r/sin(ω/2) from the apex, and it touches each side a distance r/tan(ω/2) from the apex,
where ω is the corner's angle, 44.3°: cos ω = 123/√(153·193).) The round meets each flat end in
such an arc.
