+++
title = "A chain of three balls halved through their centers"
tags = ["boolean", "union", "cut", "sphere", "planar", "analytic"]
origin = "brep-conformance 1.0.0, booleans/sequences/sphere-clusters"

[volume]
value = 429.8895547312204
exact = "half of boolean/chain-of-three-balls"
tex = '\tfrac{1}{2}\left(\tfrac{4}{3}\pi\left(5^3 + 4^3 + 3^3\right) - \sum_{\text{caps}} \tfrac{\pi}{3}h^2(3r - h)\right)'

[area]
value = 407.7455565115006
exact = "half of boolean/chain-of-three-balls's, and the cut: π(5² + 4² + 3²) less the two lenses where the disks overlap, Σ r²·acos(a/r) − a√(r² − a²) over each lens's two segments, a = 29/7 and 20/7 (radii 5, 4), 16/5 and 9/5 (radii 4, 3)"
tex = '\tfrac{1}{2}\left(4\pi\left(5^2 + 4^2 + 3^2\right) - \sum_{\text{caps}} 2\pi r h\right) + \pi\left(5^2 + 4^2 + 3^2\right) - \sum_{\text{segments}} \left(r^2 \arccos\tfrac{a}{r} - a\sqrt{r^2 - a^2}\right)'
+++

The chain of `boolean/chain-of-three-balls` (balls of radius 5, 4 and 3 centered on the origin,
(7, 0, 0) and (10, 4, 0), united one after another), cut by the plane z = 0 through all three
centers: keep the part with z ≥ 0.

The cut plane passes through each ball's center and through the middle of each circle where two
spheres cross, so it meets those circles square, at their diameters' ends. Its face is the union
of three disks of radius 5, 4 and 3, two pairs of them overlapping.

**As a set:**

$$\Bigl(B_5 \cup B_4\bigl((7, 0, 0)\bigr) \cup B_3\bigl((10, 4, 0)\bigr)\Bigr) \cap \{\, z \ge 0 \,\}$$
