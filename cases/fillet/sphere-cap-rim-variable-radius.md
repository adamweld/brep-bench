+++
title = "A spherical cap's flat rim rounded with a radius that varies all the way round it"
tags = ["fillet", "variable-radius", "sphere", "closed-edge", "analytic"]
origin = "OCCT tests/blend/buildevol/E6"

[volume]
value = 335.74305437015476
exact = "416π/3 − ∫₀^{2π} ∫₆^{z_T} (100 − z² − (ρ_c + √(r² − (z − 6 − r)²))²)/2 dz dφ, the corner removed in each plane through the axis taken between the sphere ρ² + z² = 100 and the round’s arc, from the flat z = 6 up to the tangent point z_T = 10(6 + r)/(10 − r); ρ_c = √(64 − 32r), r = 5/4 + (cos φ)/4"
tex = '\tfrac{416\pi}{3} - \int_0^{2\pi}\!\!\int_6^{z_T}\tfrac12\Bigl(100 - z^2 - \bigl(\rho_c + \sqrt{r^2 - (z - 6 - r)^2}\bigr)^2\Bigr)dz\,d\varphi,\quad \rho_c = \sqrt{64 - 32r},\ z_T = \tfrac{10(6 + r)}{10 - r}'

[area]
value = 297.12251902042132
exact = "144π − ∫₀^{2π}(16r + 100((6 + r)/(10 − r) − 3/5))dφ + A_round (the flat loses the band (64 − ρ_c²)/2 = 16r per radian, the sphere the band 100(z_T/10 − 3/5)) where A_round = ∫∫|P_φ × P_ψ|dψdφ, P(φ,ψ) = ((ρ_c + r cos ψ)cos φ, (ρ_c + r cos ψ)sin φ, 6 + r + r sin ψ), ψ ∈ [−π/2, atan2(6 + r, ρ_c)]"
tex = '144\pi - \int_0^{2\pi}\Bigl(16r + 100\bigl(\tfrac{6 + r}{10 - r} - \tfrac35\bigr)\Bigr)d\varphi + \int_0^{2\pi}\!\!\int_{-\pi/2}^{\psi_T}\lvert P_\varphi \times P_\psi\rvert\,d\psi\,d\varphi,\quad \psi_T = \operatorname{atan2}(6 + r, \rho_c)'
+++

A cap of a ball: the part of the ball of radius 10 about the origin with z ≥ 6, a dome 4 high on a
flat circular face of radius 8 in the plane z = 6. Round the circular edge where the flat face meets
the sphere with a radius that varies all the way round it: at the angle φ about the z axis,
measured from +x toward +y, r = 5/4 + (cos φ)/4. So the radius is 1.5 at φ = 0, where the edge's
seam is, 1 opposite it at φ = 180°, and varies smoothly between.

The round's sections square to the edge are circular: in the plane through the z axis at each angle
φ, the round's section is an arc of the circle of radius r(φ) that touches the flat face and the
sphere from inside. Its center is at (ρ, z) = (√(64 − 32r), 6 + r), r above the flat and 10 − r
from the ball's center. It touches the flat at (√(64 − 32r), 6) and the sphere at the point where
the ray from the ball's center through the circle's center reaches the sphere. The edge's angle
between the flat and the sphere is 53.13°.
