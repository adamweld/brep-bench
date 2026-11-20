+++
title = "A prism on a closed Bézier curve, shelled with its top open"
tags = ["shell", "inward", "bezier", "freeform", "analytic"]
origin = "OCCT tests/bugs/modalg_2/bug5805_17, OCCT tests/bugs/modalg_2/bug5805_18, OCCT tests/bugs/modalg_2/bug5805_19, OCCT tests/bugs/modalg_2/bug5805_20"

[volume]
value = 165.07259978303121
exact = "A·h − (A − pt + πt²)(h − t), A = 442/5, p = 4∫₀¹|B′(s)|ds ≈ 33.80348960226989, h = 8, t = 1/2: the prism less its cavity"
tex = 'A h - \left(A - p t + \pi t^2\right)(h - t),\quad A = \tfrac{442}{5},\ p = 4\int_0^1 |B^{\prime}(s)|\,ds,\ h = 8,\ t = \tfrac12'

[area]
value = 677.1921439332598
exact = "p·h + 2A + (p − 2πt)(h − t): the outside, side p·h and bottom A; the rim and the cavity's floor, together A again (the top outline's inside); the cavity's side (p − 2πt)(h − t)"
tex = 'p h + 2A + (p - 2\pi t)(h - t)'
+++

A prism on a closed curve of four cubic Bézier arcs. The first arc has control points (5, 0),
(5, 4), (4, 5), (0, 5), from the point (5, 0) to the point (0, 5), leaving (5, 0) straight up
and arriving at (0, 5) straight in toward −x. The other three are that arc turned 90°, 180° and 270°
about the z axis, so that the four make one smooth closed curve, a square with swollen sides, and
its inside is convex. The prism is the inside of that curve in the plane z = 0 extruded straight
up to z = 8: a flat bottom, a flat top and one freeform side of four patches. Its section has
area 442/5 and its curve length is p = 4∫|B′| ≈ 33.80349 (B′ the derivative of the first arc).

Shell it inward to a wall 1/2 thick, open at its top face, z = 8. The outside of the wall is its
side and its bottom, unmoved, and the inside is each of them moved 1/2 inward along its normal,
onto the surface offset 1/2 from it. The moved faces are extended or trimmed where they meet one
another, and run on until they meet the surface of the opening, which is left as a rim between the
outside and the inside.

The curve is at its tightest where its radius of curvature is about 2.39, more than 1/2, so the
curve 1/2 inside it is smooth, and the side moved in is its extrusion; the bottom moved in is the
plane z = 1/2. The result is the prism less the solid over the inner curve from z = 1/2 to z = 8,
a cup. The area inside the inner curve is A − pt + πt² and its length p − 2πt (Steiner), t = 1/2.

**As a set:**

$$\{\,(x, y) \text{ inside the curve},\ 0 \le z \le 8\,\} \setminus \bigl(\{\,(x, y) \text{ inside the curve},\ z \ge \tfrac12\,\} \ominus B_{1/2}\bigr)$$
