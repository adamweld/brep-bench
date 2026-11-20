+++
title = "A double cone on a closed Bézier curve"
tags = ["loft", "ruled", "point-ends", "bezier", "unsettled"]
origin = "OCCT thrusection/specific/C7"
settled = false
+++

Two cones joined base to base, as a ruled loft from a point through a closed Bézier curve to a second point.

The curve is the cubic Bézier curve in the plane z = 0 with control points (−3, 0, 0), (5, −6, 0), (5, 6, 0) and (−3, 0, 0): its two ends meet at (−3, 0, 0), where it has a corner. It is a teardrop, simple and closed, passing through (3, 0, 0). The two points are (1, 0, 3) above it and (1, 0, −3) below it, over the inside of the teardrop. Each point is joined to every point of the curve by a straight line, so the solid is two cones on the teardrop. The teardrop encloses the area 72/5, so each cone has volume ⅓·(72/5)·3 and the solid 144/5; the area of the cones has no closed form.
