+++
title = "A block's top face turned 60° about a bottom edge and joined to it"
tags = ["feature", "revolve", "union", "cylinder", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestRevolve.py testRevolveFace"

[volume]
value = 3047.197551196598
exact = "abL + θ(r₂² − r₁²)L/2, a = b = 10, L = 20, θ = π/3, r₁ = 10, r₂ = 10√2: = 2000 + 1000π/3 (each point of the face, r from the line, sweeps an arc rθ)"
tex = 'abL + \tfrac{\theta}{2}\left(r_2^2 - r_1^2\right)L,\quad a = b = 10,\ L = 20,\ \theta = \tfrac{\pi}{3},\ r_1 = 10,\ r_2 = 10\sqrt{2}:\ = 2000 + \tfrac{1000\pi}{3}'

[area]
value = 1610.3514612362037
exact = "800 the block less its top, 2·θ(r₂² − r₁²)/2 the sweep's ends, r₂θL + r₁θL its cylinders, 10·20 its last face: = 1000 + 100π + 200√2π/3"
tex = '800 + \theta\left(r_2^2 - r_1^2\right) + (r_1 + r_2)\,\theta L + 200 = 1000 + 100\pi + \tfrac{200\sqrt{2}\,\pi}{3}'
+++

A block from the origin to (10, 20, 10), joined to the solid its top face sweeps when the face is
turned 60° about the line of the block's bottom edge at x = 0, z = 0 (the y axis), over toward −x.

The top face is the rectangle 0 ≤ x ≤ 10, 0 ≤ y ≤ 20 at z = 10. The turn is about the y axis, by
60° in the sense that carries +z toward −x (a turn of −60° about +y), so the face lifts off the
block and tips over its edge x = 0.

In the plane y = 0 the face is the segment from (x, z) = (0, 10) to (10, 10). Its nearest point to
the line is its end (0, 10), 10 from it, and its farthest the end (10, 10), 10√2 from it. So the
sweep is bounded by the face where it starts (the block's top, which the two share), the face where
it stops, the cylinder of radius 10 about the y axis traced by the edge x = 0, z = 10, the cylinder
of radius 10√2 traced by the edge x = 10, z = 10, and its two ends in the planes y = 0 and y = 20,
flush with the block's.

Every point of the sweep lies above the block's top or beyond its side x = 0, so the sweep and the
block meet only in the top face, and the solid is the two joined there.

**As a set:**

$$K \cup \bigcup_{t \in [0^\circ,\, 60^\circ]} R_y(-t)\, F, \quad K = [0, 10] \times [0, 20] \times [0, 10]$$

F is the block's top face.
