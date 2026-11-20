+++
title = "A block with a helical groove screwed through it from its top to its bottom"
tags = ["sweep", "helix", "screw", "groove", "cut", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestTopologicalNamingProblem.py::testPartDesignElementMapSubHelix"

[volume]
value = 4498.40710525538
exact = "20·20·12 − (12/4 turns)·2·π(5² − 3²) = 4800 − 96π"
tex = '20\cdot 20\cdot 12 - 3\cdot 2\cdot\pi(5^2 - 3^2) = 4800 - 96\pi'

[area]
value = 2316.795618948185
exact = "sides 4·20·12 + top and bottom 2(400 − 8π) + groove floor and roof 2π·3·2·(3 + 5) + the two helicoid walls 2·2π·3·[F(5) − F(3)], c = 4/(2π), F(ρ) = (ρ√(ρ² + c²) + c² asinh(ρ/c))/2"
tex = '960 + 2(400 - 8\pi) + 2\pi\cdot 3\cdot 2\,(3 + 5) + 2\cdot 2\pi\cdot 3\,[F(5) - F(3)],\quad c = \tfrac{4}{2\pi}'
+++

A block from (−10, −10, 0) to (10, 10, 12), with a helical groove of square section cut through it from top to bottom.

The groove is cut by the square 3 ≤ x ≤ 5, −3 ≤ z ≤ −1 in the plane y = 0, screwed about the z axis: turned right-handed (from +x toward +y) while it rises 4 for every turn, through four turns, so that it ends in the same plane as the square 3 ≤ x ≤ 5, 13 ≤ z ≤ 15. Cut from the block everything the square passes through.

Each point of the square keeps its distance from the axis, so the groove's floor and roof are cylinders of radius 3 and 5, and its walls are helicoids. The turns are 4 apart while the groove is 2 high, so they leave 2 of solid between them. The groove starts below the block and ends above it, so it opens in the bottom face and in the top face, each opening a half-turn sector of the ring 3 ≤ ρ ≤ 5, and has no end faces inside the block.

**As a set:**

$$[-10,10]^2\times[0,12] \setminus \bigcup_{t \in [0,4]} T(4t)\,R_z(360^\circ t)\,Q$$

Q is the square, T(s) the move up by s along the z axis, and R_z(α) the turn about the z axis by α.
