+++
title = "A smooth loft from a point through a periodic B-spline to a point, the points off the axis"
tags = ["loft", "smooth", "point-ends", "b-spline", "unsettled"]
origin = "OCCT thrusection/specific/E3"
settled = false
+++

A smooth loft from a point through a closed periodic B-spline curve to a second point, both points displaced sideways from the curve's axis.

The curve lies in the plane z = 0. It is the closed periodic cubic B-spline $C(u) = \sum_i P_i N(u - i)$ with five control points $P_0, \dots, P_4$ = (4, 0), (1.5, 3), (−3, 2), (−3, −2), (1.5, −3) and uniform knots at the integers (all weights 1; $N$ the uniform cubic B-spline basis function, the indices running round the five points), closed after one period, u from 0 to 5. The points are $A = (2, 1, -4)$ below it and $B = (-2, 0, 4)$ above it. The loft is the surface traced by one quadratic Bézier curve for each u: its control points are $A$, $2C(u) - (A + B)/2$ and $B$, so it runs from $A$ to $B$ and passes through $C(u)$ at its parameter ½. The solid is bounded by this one surface. Its section at every height is the curve's region scaled about its own centre by a factor $1 - z^2/16$ and moved, so its volume is $\tfrac{16}{15}\cdot 4 \cdot a$, a the area the curve encloses, but its area has no closed form.
