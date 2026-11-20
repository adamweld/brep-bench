+++
title = "A smooth loft from a point through an ellipse to a point, the points off the axis"
tags = ["loft", "smooth", "point-ends", "elliptic", "unsettled"]
origin = "OCCT thrusection/specific/C5"
settled = false
+++

A smooth loft from a point through an ellipse to a second point, both points displaced sideways from the ellipse's axis.

The ellipse lies in the plane z = 0, centered on the origin, with semi-axis 4 along x and 2 along y: $C(\theta) = (4\cos\theta,\ 2\sin\theta,\ 0)$. The points are $A = (2, 0, -3)$ below it and $B = (-1, 1, 3)$ above it. The loft is the surface traced by one quadratic Bézier curve for each θ: its control points are $A$, $2C(\theta) - (A + B)/2$ and $B$, so it runs from $A$ to $B$ and passes through $C(\theta)$ at its parameter ½. The solid is bounded by this one surface. Its section at every height z is the ellipse's region scaled by $1 - z^2/9$ and moved, so its volume is 128π/5; the area has no closed form.
