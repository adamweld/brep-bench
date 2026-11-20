+++
title = "An oblique triangular frustum shelled with its top open"
tags = ["shell", "inward", "planar", "loft", "analytic"]
origin = "OCCT tests/bugs/modalg_2/bug5805_26, OCCT tests/bugs/modalg_2/bug5805_27"

[volume]
value = 191.60655421466217
exact = "252 − (7/1152)[(135 − s)² + (72 − s)² + (135 − s)(72 − s)], s = 3√17 + 2√65 + 4√26: the frustum, 8(54 + 27/2 + √(54·27/2))/3 = 252, less its cavity, a frustum of height 7 whose end sections have areas (135 − s)²/384 and (72 − s)²/384"
tex = '252 - \tfrac{7}{1152}\left[(135 - s)^2 + (72 - s)^2 + (135 - s)(72 - s)\right],\quad s = 3\sqrt{17} + 2\sqrt{65} + 4\sqrt{26}'

[area]
value = 402.76299461540936
exact = "54 + L + 27/2 − (72 − s)²/384 + (135 − s)²/384 + (4/3)L((135 − s)² − (72 − s)²)/144², L = 27√17/2 + 9√65 + 18√26: the bottom, the three sides (L), the rim, the cavity's floor and its three sides, similar in their planes to the outer sides"
tex = '54 + L + \tfrac{27}{2} - \tfrac{(72 - s)^2}{384} + \tfrac{(135 - s)^2}{384} + \tfrac{4}{3}L\,\tfrac{(135 - s)^2 - (72 - s)^2}{144^2},\quad L = \tfrac{27\sqrt{17}}{2} + 9\sqrt{65} + 18\sqrt{26}'
+++

A frustum of a triangular pyramid, lofted straight between two triangles: the triangle with
corners (0, 0, 0), (12, 0, 0), (0, 9, 0) in the plane z = 0, and the triangle with corners
(2, 1, 8), (8, 1, 8), (2, 11/2, 8) in the plane z = 8, half its size with its sides parallel to
the lower one's, standing off to one side. Its three sides are planar trapezoids, each leaning
(the pairs of corners joined are (0, 0, 0) to (2, 1, 8), (12, 0, 0) to (8, 1, 8) and (0, 9, 0)
to (2, 11/2, 8)), and it is closed by the two triangles.

Shell it inward to a wall 1 thick, open at its top face, z = 8. The outside of the wall is its
sides and its bottom, unmoved, and the inside is each of them moved 1 inward along its normal,
onto the plane offset 1 from it. The moved faces are extended or trimmed where they meet one
another, and run on until they meet the plane of the opening, which is left as a rim between the
outside and the inside; all the edges are convex, so the inside's edges are sharp.

The sides' outward unit normals are (0, −8, 1)/√65 on the side over the edge from (0, 0, 0) to
(12, 0, 0), (3, 4, 1)/√26 on the side over the edge from (12, 0, 0) to (0, 9, 0), and
(−4, 0, 1)/√17 on the side over the edge from (0, 9, 0) to (0, 0, 0). The cavity is the part, from
z = 1 up to z = 8, of the solid left under those three planes moved 1 inward and over the bottom
moved to z = 1: a smaller frustum of a pyramid, sections at z = 1 and z = 8 of areas about 19.31
and 1.39, the walls leaning as the outside's do. The result is a cup of the frustum less that.

**As a set:** with n₁, n₂, n₃ the unit normals above and d₁, d₂, d₃ their offsets (the planes
n·p = d through a side),

$$\{\,n_i \cdot p \le d_i,\ 0 \le z \le 8\,\} \setminus \{\,n_i \cdot p \le d_i - 1,\ 1 \le z \le 8\,\}$$

with d₁ = 0, d₂ = 36/√26 and d₃ = 0.
