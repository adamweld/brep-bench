+++
title = "A crescent turned three quarters of a turn about the line through its tips"
tags = ["primitive", "revolve", "pappus", "degenerate-edge"]
origin = "OCCT tests/bugs/modalg_7/bug29586"

[volume]
value = 1110.3304951225528
exact = "(3π/2)·(75π) = 225π²/2: Pappus, the turn 3π/2 times the crescent's first moment about the axis, which is 3·25π, the moment of the disc of radius 5 about the axis, since the lens taken from it is symmetric about the axis"
tex = '\tfrac{225\pi^2}{2}'

[area]
value = 671.28688134667179
exact = "2(25π + 24 − 50 atan(4/3)) the two end faces + (3π/2)(30π − 60 atan(4/3) + 80) the faces of revolution (Pappus over the two arcs) = 45π² + 170π + 48 − (100 + 90π) atan(4/3)"
tex = '45\pi^2 + 170\pi + 48 - (100 + 90\pi)\arctan\tfrac43'
+++

In the xz plane, take the disc of radius 5 centered at (0, 0) less the disc of radius 5 centered at
(6, 0): a crescent. The two circles cross at (3, ±4), the crescent's two tips, and the crescent lies
wholly in x ≤ 3. Turn it about the line x = 3, z = 0, parallel to the y axis (the line through
the tips), by 270°, right-handed about +y: the crescent's far side, which starts on the −x side of
the line, goes first toward +z, then +x, and stops at −z. The solid's ends are two copies of the
crescent, and its two tips stay on the axis.

**As a set:**

$$\{\,(3 - \rho,\ y) \in C,\ 0 \le \psi \le 270^\circ\,\}$$

where ρ and ψ are the distance from the axis and the angle turned from its −x side (toward +z), and
C = {(u, y): u² + y² ≤ 25 and (u − 6)² + y² > 25} is the crescent.
