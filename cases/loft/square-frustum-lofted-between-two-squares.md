+++
title = "A square frustum lofted between two squares"
tags = ["loft", "ruled", "planar", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestPipe.py::testMultisectionPipeSketchEdgeSection"

[volume]
value = 69.33333333333333
exact = "(h/3)(A₁ + A₂ + √(A₁A₂)), h = 4, A₁ = 36, A₂ = 4: (4/3)(36 + 4 + 12) = 208/3"
tex = '\tfrac{h}{3}\left(A_1 + A_2 + \sqrt{A_1 A_2}\right) = \tfrac{4}{3}(36 + 4 + 12) = \tfrac{208}{3}'

[area]
value = 111.55417527999327
exact = "36 + 4 + four trapezoids of parallel sides 6 and 2 and height √(4² + 2²) = 2√5: 40 + 4·4·2√5 = 40 + 32√5"
tex = '36 + 4 + 4\cdot\tfrac{6 + 2}{2}\cdot 2\sqrt{5} = 40 + 32\sqrt{5}'
+++

A solid lofted along the z axis between two squares, joined by flat ruled faces: the square of side 6
in the plane z = 0, from (−3, −3, 0) to (3, 3, 0), and the square of side 2 in the plane z = 4, from
(−1, −1, 4) to (1, 1, 4), with their sides parallel and corners paired (3, 3, 0) with (1, 1, 4) and
so on, so that the loft is not twisted. It is a square frustum: the convex hull of the two squares,
bounded by them and four trapezoids.

**As a set:**

$$\operatorname{conv}\bigl([-3,3]^2 \times \{0\} \,\cup\, [-1,1]^2 \times \{4\}\bigr)$$
