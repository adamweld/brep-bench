+++
title = "A rod with a helical groove cut by a bar whose outer face lies on the rod's side"
tags = ["sweep", "helix", "screw", "groove", "cut", "coincident", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestHelix.py::testGiantHelixSubtractive"

[volume]
value = 2026.3272615654166
exact = "π6²·20 − A·2π·ρ̄·n, A = 2·1.5, ρ̄ = 5, n = 2.5: = 720π − 75π = 645π"
tex = '\pi\cdot 6^2\cdot 20 - A\cdot 2\pi\bar\rho\, n = 720\pi - 3\cdot 2\pi\cdot 5\cdot 2.5 = 645\pi'

[area]
value = 1255.7824572947566
exact = "the rod 2π·6² + 2π·6·20, less the groove's mouth on its side 2π·6·1.5·n, plus the groove's floor 2π·4·1.5·n, its two ends 2·(2·1.5), and its two walls, helicoids 2·2πn[F(6) − F(4)], c = 4/(2π), F(ρ) = (ρ√(ρ² + c²) + c² asinh(ρ/c))/2"
tex = '2\pi\cdot 6^2 + 2\pi\cdot 6\cdot 20 - 2\pi n\cdot 6\cdot 1.5 + 2\pi n\cdot 4\cdot 1.5 + 6 + 2\cdot 2\pi n\,[F(6) - F(4)],\quad n = 2.5,\ c = \tfrac{4}{2\pi}'
+++

A rod with a helical groove of rectangular section cut into its side, the groove's outer face lying on the rod's own cylinder.

The rod is the cylinder of radius 6 about the z axis from z = 0 to z = 20.

The groove is cut by the rectangle 4 ≤ x ≤ 6, 4 ≤ z ≤ 5.5 in the plane y = 0, its outer side on the rod's surface, screwed about the z axis: turned right-handed (from +x toward +y) while it rises 4 for every turn, through two and a half turns (900°). It ends in the plane y = 0 on the other side of the axis, as the rectangle −6 ≤ x ≤ −4, 14 ≤ z ≤ 15.5. Cut from the rod everything the rectangle passes through.

Each point of the rectangle keeps its distance from the axis. So the groove's floor lies on the cylinder of radius 4, its walls are helicoids, and its mouth is a helical band 1.5 wide on the rod's side. Its turns are 4 apart while it is 1.5 wide, so they leave 2.5 of the rod's side between them. Its two ends are flat rectangles in the plane y = 0, and lie wholly inside the rod.

**As a set:**

$$\bigl\{\,\rho \le 6,\ 0 \le z \le 20\,\bigr\} \setminus \bigcup_{t \in [0,\,2.5]} T(4t)\, R_z(360^\circ t)\, Q$$

Q is the rectangle, T(s) the move up by s along the z axis, and R_z(α) the turn about the z axis by α.
