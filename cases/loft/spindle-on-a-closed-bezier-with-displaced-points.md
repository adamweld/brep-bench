+++
title = "A smooth loft from a point through a closed Bézier curve to a point, the points off the axis"
tags = ["loft", "smooth", "point-ends", "bezier", "unsettled"]
origin = "OCCT thrusection/specific/C9"
settled = false
+++

A smooth loft from a point through a closed Bézier curve to a second point, both points displaced sideways from the curve's middle.

The curve is the cubic Bézier curve $C(t)$ in the plane z = 0 with control points (−3, 0, 0), (5, −6, 0), (5, 6, 0) and (−3, 0, 0), a teardrop with a corner at (−3, 0, 0) enclosing the area 72/5. The points are $A = (2, 1, -3)$ below it and $B = (0, -1, 3)$ above it. The loft is the surface traced by one quadratic Bézier curve for each t in [0, 1]: its control points are $A$, $2C(t) - (A + B)/2$ and $B$, so it runs from $A$ to $B$ and passes through $C(t)$ at its parameter ½. The solid is bounded by this one surface. Its section at every height z is the teardrop scaled by $1 - z^2/9$ and moved, so its volume is $\tfrac{16}{15}\cdot 3\cdot\tfrac{72}{5} = 1152/25$; the area has no closed form.
