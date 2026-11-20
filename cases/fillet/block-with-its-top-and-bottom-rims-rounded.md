+++
title = "A block with the rims of its top and bottom rounded, its upright edges sharp"
tags = ["fillet", "constant-radius", "convex", "mirror", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestMultiTransform.py::testMultiTransformDressup"

[volume]
value = 3868.79052632616
exact = "20·20·10 − 8(1 − π/4)·2²·20 + 8(5/3 − π/2)·2³: eight rounded edges of length 20 each remove (1 − π/4)r², and the eight corners where two rounds overlap are counted twice, each overlap ∫₀ʳ (r − √(r² − u²))² du = (5/3 − π/2)r³ = 10400/3 + 128π"
tex = '3360 + \tfrac{320}{3} + 128\pi = \tfrac{10400}{3} + 128\pi'

[area]
value = 1458.1238596594935
exact = "two flat faces 16·16, four flat side faces 20·6, eight round patches each 2·∫₀^{π/2} 2(8 + 2cos θ) dθ = 16π + 8: 512 + 480 + 128π + 64 = 1056 + 128π"
tex = '1056 + 128\pi'
+++

A block with the rims of its top and bottom faces rounded and its four upright edges left sharp.

The block is [−10, 10] × [−10, 10] × [−5, 5]. The eight edges that bound its top face z = 5 and its bottom face z = −5 are
each rounded with radius 2 (a quarter-cylinder of radius 2 replacing the edge). The four upright edges
are not rounded. Where two rounds meet at an upright edge (at each of the eight corners) each runs on to
the side face it meets, so the two quarter-cylinders (axes along x and along y) cut one another in a
curve there, and the corner has no round of its own.

So the top and bottom faces are the 16 × 16 squares |x|, |y| ≤ 8, and each side face is the 20 × 6 rectangle left between its rounds.

**As a set:** with Q_x the 20 × 10 rectangle in (x, z) with its four corners rounded by 2, and Q_y the
same in (y, z),

$$\{\,(x, z) \in Q_x\,\} \cap \{\,(y, z) \in Q_y\,\}$$
