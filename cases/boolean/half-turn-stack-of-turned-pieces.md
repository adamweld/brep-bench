+++
title = "A half-turn of a stacked turned part"
tags = ["boolean", "union", "revolve", "cone", "torus", "sphere", "analytic"]
origin = "OCCT boolean/gdml_public/A9"

[volume]
value = 251.17623821211788
exact = "½(98π + 5.5π² + 112π/3 + 7.2917π): half a turn of the cylinder π7²·2, the upper half of the torus ½·2π²·5.5·1², the frustum (πh/3)(16 + 8 + 4) with h = 4, and the ball's cap above z = 6, πh²(3R − h)/3 with h = 2.5, R = 2"
tex = '\tfrac12\left(98\pi + 5.5\pi^2 + \tfrac{112\pi}{3} + \tfrac{\pi\cdot 2.5^2\,(6 - 2.5)}{3}\right)'

[area]
value = 314.16619651772396
exact = "half of the full-turn skin 98.25π + 11π² + 12√5π (the pieces' 28π + 11π + 49π + ¼π + 10π and 11π² and 12√5π), plus the two flat ends, each the section S = 14 + π/2 + 12 + ½(4π − (4 arccos¼ − ½√3.75)): the cylinder's side, the rings of its top left bare, its bottom, the torus's upper half, the frustum's side, the ring of the frustum's top left bare, the ball's cap"
tex = '\tfrac12\left(98.25\pi + 11\pi^2 + 12\sqrt5\,\pi\right) + 2S'
+++

Four pieces turned about the z axis, each through half a turn only, from 0° to 180° (from +x toward +y), fused into one solid. All have flat ends in the half planes at 0° and at 180°.

- A half cylinder of radius 7 from z = 0 to z = 2.
- Half a torus about the z axis, of major radius 5.5 and tube radius 1, whose circle of centers lies in the plane z = 2. Its lower half is inside the half cylinder; the upper half stands on top, clear of everything else (it spans radii 4.5 to 6.5).
- A half cone frustum from z = 2 to z = 6, radius 4 at the bottom and 2 at the top.
- Half a ball of radius 2 centered at (0, 0, 6.5): its upper part rises above the frustum's top, its lower part, below z = 6, is inside the frustum, and it crosses the plane z = 6 in a circle of radius √3.75 ≈ 1.94, inside the frustum's top circle of radius 2.

**As a set:**

$$\{\,0 \le \varphi \le 180^\circ\,\} \cap \bigl(\{\,\rho \le 7,\ 0 \le z \le 2\,\} \cup \{\,(\rho - 5.5)^2 + (z - 2)^2 \le 1\,\} \cup \{\,\rho \le 4 - \tfrac{z - 2}{2},\ 2 \le z \le 6\,\} \cup \{\,\rho^2 + (z - 6.5)^2 \le 2^2\,\}\bigr)$$
