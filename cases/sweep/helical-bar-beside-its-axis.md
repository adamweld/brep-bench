+++
title = "A bar of rectangular section screwed round the z axis, clear of it"
tags = ["sweep", "helix", "screw", "helicoid", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestHelix.py::testGiantHelix"

[volume]
value = 622.03534541077906
exact = "A·2π·ρ̄·n, A = 3·2, ρ̄ = 5.5, n = 3: = 198π (a screw keeps each point's distance from the axis, and its rise does not change the volume)"
tex = 'A\cdot 2\pi\bar\rho\, n = 6\cdot 2\pi\cdot 5.5\cdot 3 = 198\pi'

[area]
value = 1055.3662387345469
exact = "the inside 2π·4·2·n and the outside 2π·7·2·n, two ends 2·6, and the top and bottom helicoids, each ∫₀^{2πn}∫₄⁷ √(ρ² + c²) dρ dθ = 2πn[F(7) − F(4)], c = 5/(2π), F(ρ) = (ρ√(ρ² + c²) + c² asinh(ρ/c))/2"
tex = '2\pi n\,(4\cdot 2 + 7\cdot 2) + 12 + 2\cdot 2\pi n\,[F(7) - F(4)],\quad n = 3,\ c = \tfrac{5}{2\pi},\ F(\rho) = \tfrac12\bigl(\rho\sqrt{\rho^2 + c^2} + c^2\operatorname{asinh}\tfrac{\rho}{c}\bigr)'
+++

A bar of rectangular section, standing clear of the z axis, screwed round it.

The section is the rectangle 4 ≤ x ≤ 7, 0 ≤ z ≤ 2 in the plane y = 0. It is screwed about the z axis: turned right-handed (from +x toward +y) while it rises 5 for every turn, through three whole turns, so that it ends in the plane y = 0 again as the rectangle 4 ≤ x ≤ 7, 15 ≤ z ≤ 17.

Each point of the section keeps its distance from the axis. So the bar's inside lies on the cylinder of radius 4 and its outside on the cylinder of radius 7, its top and bottom are helicoids, and its two ends are flat rectangles in the plane y = 0. Its turns are 5 apart while the section is 2 high, so they leave gaps of 3 between them.

**As a set:**

$$\bigcup_{t \in [0,\,3]} T(5t)\, R_z(360^\circ t)\, Q$$

Q is the rectangle, T(s) the move up by s along the z axis, and R_z(α) the turn about the z axis by α.
