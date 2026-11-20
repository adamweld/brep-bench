+++
title = "A double cone on a closed periodic rational B-spline"
tags = ["loft", "ruled", "point-ends", "b-spline", "rational", "unsettled"]
origin = "OCCT thrusection/specific/D6"
settled = false
+++

Two cones joined base to base, as a ruled loft from a point through a closed periodic rational B-spline curve to a second point, both points on the curve's axis.

The curve lies in the plane z = 0. It is the closed periodic cubic B-spline with five control points $P_0, \dots, P_4$ = (4, 0), (1.5, 3), (−3, 2), (−3, −2), (1.5, −3), weights 1, 1, 2, 1, 1, and uniform knots at the integers, so that $C(u) = \sum_i w_i P_i N(u - i) \big/ \sum_i w_i N(u - i)$, $N$ the uniform cubic B-spline basis function and the indices running round the five points; it closes after one period, u from 0 to 5, and is C² throughout. The two points are (0, 0, 3) and (0, 0, −3). Each point is joined to every point of the curve by a straight line, so the solid is two cones on the curve, each of volume ⅓·A·3, A the area it encloses. It has two faces, one for each cone.
