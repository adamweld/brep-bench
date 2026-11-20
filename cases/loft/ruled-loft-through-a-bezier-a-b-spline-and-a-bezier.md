+++
title = "A ruled loft through a closed Bézier curve, a periodic B-spline and a closed Bézier curve, between two points"
tags = ["loft", "ruled", "point-ends", "bezier", "b-spline", "unequal-sections", "unsettled"]
origin = "OCCT thrusection/specific/G3"
settled = false
+++

A ruled loft through three stacked closed curves of two kinds, capped by a point at each end, both points displaced sideways.

The curves: $T(t)$ is the cubic Bézier curve with control points (−3, 0), (5, −6), (5, 6), (−3, 0), a teardrop with a corner at (−3, 0), t in [0, 1]; $C(u) = \sum_i P_i N(u - i + 1)$ is the closed periodic cubic B-spline ($N$ the cubic B-spline basis function centred on 0, with knots at the integers and support $-2 < x < 2$: $N(x) = \tfrac23 - x^2 + |x|^3/2$ for $|x| \le 1$ and $(2 - |x|)^3/6$ for $1 \le |x| \le 2$; the sum runs round the five points, so $C$ has period 5, and $C(0) = (P_0 + 4P_1 + P_2)/6 = (-2.25, -1.5)$) with five control points $P_0, \dots, P_4$ = (−3, 2), (−3, −2), (1.5, −3), (4, 0), (1.5, 3), all weights 1, uniform knots at the integers, u from 0 to 5. The sections are $T$ in the plane z = 0, $C$ in the plane z = 3 and $T$ again in the plane z = 6. The end points are (1, 0.5, −3) below the first section and (0, −0.5, 9) above the last. Consecutive curves are joined by straight lines, the point $T(t)$ of a Bézier curve to the point $C(5t)$ of the spline, and each end point to every point of its neighbouring curve.
