+++
title = "A cone cut by a slanting plane"
tags = ["boolean", "cut", "cone", "analytic"]
origin = "OCCT boolean/volumemaker/E4"

[volume]
value = 147.66639517144813
exact = "(8/3)·E, E = π·(16/√15)(64/15): the cone from its apex over the cut is a third of the cut's area times the apex's distance from its plane, which is (8/3) of the cut's footprint on z = 0, the ellipse x² + (15/16)(y + 16/15)² = 256/15"
tex = '\tfrac{8}{3}E,\quad E = \pi\cdot\tfrac{16}{\sqrt{15}}\cdot\tfrac{64}{15}'

[area]
value = 185.7330548975839
exact = "∫₀^{2π} (s²/2) sin α dφ + E√5/2, with the cone's slant s = 8√5/(2 + ½ sin φ) at angle φ about the axis and sin α = 1/√5: 32√5·2π·2/(15/4)^{3/2} for the side, and the flat is the ellipse's footprint over cos θ = 2/√5"
tex = '\int_0^{2\pi}\frac{s^2}{2}\sin\alpha\,d\varphi + \tfrac{\sqrt5}{2}E,\quad s = \frac{8\sqrt5}{2 + \tfrac12\sin\varphi},\ \sin\alpha = \tfrac{1}{\sqrt5}'
+++

A cone whose apex is the point (0, 0, 12) and whose base is the disk of radius 6 in the plane z = 0, so that its side is the surface ρ = (12 − z)/2. It is cut by the plane z = 4 + y/2, and the part above the plane, with the apex, is kept.

The plane meets the cone in an ellipse that lies wholly inside its side: it reaches from y = −16/3, z = 4/3 at its lowest to y = 3.2, z = 5.6 at its highest, so the cone's base takes no part. The solid has two faces, the side of the cone from the apex down to that ellipse and the flat ellipse.

**As a set:**

$$\{\,\rho \le \tfrac{12 - z}{2},\ z \ge 4 + \tfrac{y}{2}\,\}$$
