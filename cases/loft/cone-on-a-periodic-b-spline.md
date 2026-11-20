+++
title = "A cone lofted from a closed periodic B-spline to a point"
tags = ["loft", "ruled", "point-end", "b-spline", "capped", "unsettled"]
origin = "OCCT thrusection/specific/L5"
settled = false
+++

A cone on a freeform closed curve, as a ruled loft from a closed periodic B-spline curve to a single point, the solid closed over the curve by a flat face.

The curve lies in the plane z = 0. It is the closed periodic cubic B-spline $C(u) = \sum_i P_i N(u - i)$ with five control points $P_0, \dots, P_4$ = (4, 0), (1.5, 3), (−3, 2), (−3, −2), (1.5, −3) and uniform knots at the integers (all weights 1; $N$ the uniform cubic B-spline basis function, the indices running round the five points), closed after one period, u from 0 to 5. The point is (0, 0, 4), on the axis above the curve. The point is joined to every point of the curve by a straight line, so the solid is one cone on the curve and a flat face closing it, and its volume is ⅓·A·4, A the area the curve encloses; the area of the conical face has no closed form.

**As a set:**

$$\{\,(1-s)\,(x_0, y_0, 0) + s\,(0, 0, 4) : (x_0, y_0) \in R,\ 0 \le s \le 1\,\}$$

where R is the region the curve encloses.
