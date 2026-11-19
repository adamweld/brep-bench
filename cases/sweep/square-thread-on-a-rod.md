+++
title = "A square thread wound on a rod, its inner face on the rod's side"
tags = ["sweep", "helix", "thread", "union", "coincident", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestHelix.py testGiantHelixAdditive"

[volume]
value = 6836.10561421139
exact = "πR²L + n·2π∫∫ρ dA over the square, R = 10, L = 20, n = 4: = 2000π + 8π·(2·11) = 2176π (a screw keeps each point's distance from the axis, and its rise does not change the volume)"
tex = '\pi R^2 L + 2\pi n \iint \rho\,dA = 2000\pi + 8\pi\cdot 22 = 2176\pi'

[area]
value = 3046.105815837052
exact = "the rod 2π·10² + 2π·10·20 less the band under the thread 2π·10·n·1, the thread's outside 2π·12·n·1, its two ends 2·2, and its two helicoids, each ∫₀^{2πn}∫₁₀¹² √(ρ² + c²) dρ dθ, c = 3/(2π): = 616π + 4 + 16π[F]₁₀¹², F(ρ) = (ρ√(ρ² + c²) + c² ln(ρ + √(ρ² + c²)))/2"
tex = '616\pi + 4 + 2\cdot 2\pi n\,\Bigl[\tfrac{1}{2}\rho\sqrt{\rho^2 + c^2} + \tfrac{1}{2}c^2\ln\bigl(\rho + \sqrt{\rho^2 + c^2}\bigr)\Bigr]_{10}^{12},\quad n = 4,\ c = \tfrac{3}{2\pi}'
+++

A rod with a thread of square section wound round it and joined to it, the thread's inner face
lying on the rod's side.

The rod is a cylinder of radius 10 on the z axis, from z = 0 to z = 20.

The thread is the square 10 ≤ x ≤ 12, 3 ≤ z ≤ 4 in the plane y = 0, its inner side on the rod's
surface, screwed about the z axis: turned right-handed, from +x toward +y, while it rises 3 for
every turn, through four whole turns. It ends in the plane y = 0 again, as the square
10 ≤ x ≤ 12, 15 ≤ z ≤ 16.

Each point of the square keeps its distance from the axis. So the thread's top and bottom are
helicoids, its outside lies on the cylinder of radius 12, and its inside lies on the rod's side,
along a helical band 1 wide. Its turns are 3 apart, which leaves gaps of 2 between them. Its two
ends are flat squares. The rod and the thread are one solid.

**As a set:**

$$\{\,\rho \le 10,\ 0 \le z \le 20\,\} \cup \bigcup_{t \in [0,\, 4]} T(3t)\, R_z(360^\circ t)\, Q$$

Q is the square, and T(s) the move up by s along the z axis.
