+++
title = "A turned post on the Bézier dome block, its outer side drafted"
tags = ["draft", "cone", "bezier", "surface", "revolve", "boolean"]
settled = false
origin = "OCCT tests/draft/angle/H5"
+++

The block under a bicubic Bézier dome of `nurbs/bezier-dome`: standing on the square
0 ≤ x ≤ 20, 0 ≤ y ≤ 20 of the xy plane, its floor flat at z = 0, its four sides upright in the planes
x = 0, x = 20, y = 0 and y = 20, and its top the bicubic Bézier patch whose sixteen control points are
Pᵢⱼ = (20i/3, 20j/3, zᵢⱼ), i, j = 0, 1, 2, 3, at these heights zᵢⱼ (row i, x = 20i/3; column j,
y = 20j/3):

| | j = 0 | j = 1 | j = 2 | j = 3 |
|---|---|---|---|---|
| i = 0 | 4 | 6 | 7 | 5 |
| i = 1 | 6 | 14 | 12 | 7 |
| i = 2 | 5 | 13 | 16 | 8 |
| i = 3 | 3 | 6 | 8 | 6 |

The patch is S(u, v) = Σᵢ Σⱼ Bᵢ(u) Bⱼ(v) Pᵢⱼ for u, v from 0 to 1, with the cubic Bernstein
polynomials B₀(t) = (1 − t)³, B₁(t) = 3t(1 − t)², B₂(t) = 3t²(1 − t), B₃(t) = t³, so that x = 20u
and y = 20v.

A post stands on it, turned about the vertical line through (10, 10): the region between 2 and 5
out from that line, 0 ≤ z ≤ 26, of the half plane y = 10, x > 10, turned 150° counterclockwise
(seen from +z) about the line from +x. It is a curved block with an outer face on the cylinder of
radius 5 about the line, an inner face on the cylinder of radius 2, a flat top at z = 26 and two
flat ends in the vertical planes through the line at 0° and 150°. It is united with the block; it
shows only above the dome.

Draft the post's outer face by 4°, with the pull direction +z and the neutral plane z = 0, the
block's floor: the face turns about the arc of radius 5 where it crosses the neutral plane, until it
leans 4° from the z direction, inward as it rises, into the cone of radius 5 − z·tan 4° about the
vertical line. The inner face, the two ends, the top and the block are not drafted; the top is cut
back to meet the cone. It has no closed form that has been checked.

**As a set:**

$$\{\, 0 \le z \le S(x, y) \,\} \cup \{\, 2 \le \sqrt{(x-10)^2 + (y-10)^2} \le 5 - z\tan 4^\circ,\ 0 \le z \le 26,\ 0^\circ \le \varphi \le 150^\circ \,\}$$

where S(x, y) is the patch's height over the point (x, y) of the square, and φ is the angle about
the vertical line through (10, 10), from +x counterclockwise.
