+++
title = "A smooth loft from a tiny periodic B-spline through a large one to a tiny one"
tags = ["loft", "smooth", "b-spline", "unequal-sections", "unsettled"]
origin = "OCCT thrusection/specific/F6"
settled = false
+++

A smooth loft through three closed periodic B-spline curves, the middle one much larger than the two at the ends, all on one axis.

The middle curve lies in the plane z = 0. It is the closed periodic cubic B-spline $C(u) = \sum_i P_i N(u - i)$ with five control points $P_0, \dots, P_4$ = (4, 0), (1.5, 3), (−3, 2), (−3, −2), (1.5, −3) and uniform knots at the integers (all weights 1; $N$ the uniform cubic B-spline basis function, the indices running round the five points), closed after one period, u from 0 to 5. The two end curves are its tiny copies $T(u) = C(u)/8$ (in x and y) in the planes z = −3 and z = 3. The loft is the surface traced by one quadratic Bézier curve for each u: its control points are $T(u)$ at z = −3, $2C(u) - T(u)$ at z = 0 and $T(u)$ at z = 3, so it runs from the lower curve to the upper and passes through $C(u)$ at its parameter ½. The solid is bounded by this surface and the two flat regions the end curves enclose. Its section at height z = 3s is $(1 - 7s^2/8)\,C$, so its volume is 547a/160, a the area $C$ encloses; the area has no closed form.
