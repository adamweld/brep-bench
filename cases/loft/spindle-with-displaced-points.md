+++
title = "A spindle whose points are off the axis"
tags = ["loft", "smooth", "point-ends", "unsettled"]
origin = "OCCT thrusection/specific/B6"
settled = false
+++

A smooth loft from a point through a circle to a second point, both points displaced sideways from the circle's axis: the spindle of a smooth loft with its ends moved off the axis.

The circle has radius 3, lies in the plane z = 0 and is centered on the z axis. The points are $A = (2, 0, -4)$ below it and $B = (-1, 2, 4)$ above it. The loft is the surface traced by one quadratic Bézier curve for each point $C = (3\cos\theta,\ 3\sin\theta,\ 0)$ of the circle: its control points are $A$, $2C - (A + B)/2$ and $B$, so it runs from $A$ to $B$ and passes through $C$ at its parameter ½. The solid is pointed at both ends, bounded by this one surface. Its section at every height is a circle of radius $3(1 - z^2/16)$, so its volume is 192π/5, that of the spindle with its points on the axis; the area has no closed form.
