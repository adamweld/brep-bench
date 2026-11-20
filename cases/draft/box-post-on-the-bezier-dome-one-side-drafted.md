+++
title = "A square post on the Bézier dome block, one side drafted"
tags = ["draft", "planar", "bezier", "surface", "boolean"]
settled = false
origin = "OCCT tests/draft/angle/G9"
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

A post stands on it: the box [6, 12] × [6, 12] × [0, 26], united with the block. Its sides show
only above the dome, which is between about 8.8 and 10.6 high over the box; its top, at z = 26, is
above the dome everywhere.

Draft the post's side face in the plane x = 12 by 4°, with the pull direction +z and the neutral
plane z = 0, the block's floor: the face turns about the line x = 12, z = 0, until it leans 4° from
the z direction, toward the post's inside as it rises. It becomes the plane x = 12 − z·tan 4°. The
post's other three sides, its top and the block are not drafted; the top and the two sides next to
the drafted face are cut back to meet it. It has no closed form that has been checked.

**As a set:**

$$\{\, 0 \le z \le S(x, y) \,\} \cup \bigl(\,[6,12]^2 \times [0,26] \cap \{\, x \le 12 - z\tan 4^\circ \,\}\bigr)$$

where S(x, y) is the patch's height over the point (x, y) of the square.
