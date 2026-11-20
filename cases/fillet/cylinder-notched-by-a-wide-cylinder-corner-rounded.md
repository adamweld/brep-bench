+++
title = "A cylinder notched by a wider, shorter one, one edge of the notch rounded"
tags = ["fillet", "cylinder", "convex", "analytic"]
origin = "OCCT tests/bugs/modalg_5/bug25657_1"

[volume]
value = 3687.759011052443
exact = "V = (100π − L − A)·8 + 100π·4, L = 100α + 36β − 140 sin α the lens where the two discs overlap, α = arccos(260/280), β = arccos(132/168), and A = 0.19535681255467693 the plan area between the corner and the round (Green's theorem over its three arcs)"
tex = 'V = (100\pi - L - A)\cdot 8 + 100\pi\cdot 4'

[area]
value = 1383.8940955802502
exact = "area = 100π (top) + (100π − L − A) (floor of the notch's plan) + (L + A) (the notch's ceiling) + the lateral faces: (R(2π − 2α) − 1.2511)·8 + 2π·10·4 for the tall wall, (6·2β − 0.9695)·8 for the notch wall and 2.0214·8 for the round"
tex = '100\pi + 100\pi + 8\ell_{tall} + 8\ell_{notch} + 8\ell_{round} + 80\pi'
+++

A tall cylinder of radius 10 about the z axis, from z = 0 to z = 12.

From it, cut a wider, shorter cylinder of radius 6 about the vertical line through (14, 0), from z = 0 to z = 8: it takes a notch from the tall cylinder's side, open at the bottom, with a ceiling at z = 8 above it. The tall wall and the notch wall meet along two vertical straight edges, at (x, y) = (260/28, ±3.7115), from z = 0 to z = 8.

Round the one of those two edges that has y > 0, to radius 2: the round runs the whole length of the edge, from the bottom face to the notch's ceiling, and is the cylinder of radius 2 about the vertical line through (7, √15), tangent to the tall wall at (8.75, 4.8412) and to the notch wall at (8.75, 2.9047), the plan of the solid's corner rounded off there. The other edge, at y < 0, is left sharp.

**As a set:** the solid is the tall cylinder less the notch, and for z below 8 also less the part of the plan in the wedge at (7, √15) between the directions to (8.75, 4.8412) and to (8.75, 2.9047), outside the circle of radius 2 about (7, √15).
