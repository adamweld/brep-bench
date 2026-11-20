+++
title = "A ruled loft from a tiny periodic B-spline through a large one to a tiny one"
tags = ["loft", "ruled", "b-spline", "unequal-sections", "unsettled"]
origin = "OCCT thrusection/specific/F4"
settled = false
+++

A ruled loft through three closed periodic B-spline curves, the middle one much larger than the two at the ends, all on one axis.

The middle curve lies in the plane z = 0. It is the closed periodic cubic B-spline $C(u) = \sum_i P_i N(u - i)$ with five control points $P_0, \dots, P_4$ = (4, 0), (1.5, 3), (−3, 2), (−3, −2), (1.5, −3) and uniform knots at the integers (all weights 1; $N$ the uniform cubic B-spline basis function, the indices running round the five points), closed after one period, u from 0 to 5. The two end curves are its tiny copies, the same B-spline with every control point brought to one eighth of its distance from the z axis: $C(u)/8$ (in x and y) in the planes z = −3 and z = 3. The point of an end curve at the parameter u is joined by a straight line to the point $C(u)$ of the middle curve. The solid is bounded by these two ruled surfaces and the two flat regions the end curves enclose. Each ruled surface has the section $((1 - t)/8 + t)\,C(u)$ at its fraction t from the end curve to the middle one, so the volume is 73a/32, a the area $C$ encloses; the area of the solid has no closed form.
