+++
title = "A square bar screwed round the z axis with one side lying along it"
tags = ["sweep", "helix", "screw", "helicoid", "analytic", "singular"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestHelix.py::testRectangle"

[volume]
value = 254.46900494077325
exact = "A·2π·ρ̄·n, A = 9, ρ̄ = 1.5, n = 3: = 81π"
tex = 'A\cdot 2\pi\bar\rho\, n = 9\cdot 2\pi\cdot 1.5\cdot 3 = 81\pi'

[area]
value = 378.29212384287963
exact = "the outside 2π·3·3·n, two ends 2·9, the top and bottom helicoids 2·2πn[F(3) − F(0)], c = 4/(2π), F(ρ) = (ρ√(ρ² + c²) + c² asinh(ρ/c))/2; the side on the axis has no area"
tex = '2\pi n\cdot 3\cdot 3 + 18 + 2\cdot 2\pi n\,[F(3) - F(0)],\quad n = 3,\ c = \tfrac{4}{2\pi}'
+++

A square bar screwed round the z axis with one of its sides lying along the axis.

The section is the square 0 ≤ x ≤ 3, 0 ≤ z ≤ 3 in the plane y = 0, its side x = 0 on the z axis. It is screwed about the z axis: turned right-handed (from +x toward +y) while it rises 4 for every turn, through three whole turns, so that it ends in the plane y = 0 again as the square 0 ≤ x ≤ 3, 12 ≤ z ≤ 15.

Each point of the section keeps its distance from the axis. So the bar's outside lies on the cylinder of radius 3, its top and bottom are helicoids, and its two ends are flat squares in the plane y = 0. The bar touches the axis along its whole length: the solid there is a wedge, 270° round (3 of every 4 of a turn's rise), whose edge is the axis from z = 0 to z = 15. Its turns are 4 apart while the section is 3 high, so they leave gaps of 1 between them.

**As a set:**

$$\bigcup_{t \in [0,\,3]} T(4t)\, R_z(360^\circ t)\, Q$$

Q is the square, T(s) the move up by s along the z axis, and R_z(α) the turn about the z axis by α.
