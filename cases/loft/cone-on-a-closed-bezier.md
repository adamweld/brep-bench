+++
title = "A cone lofted from a closed Bézier curve to a point"
tags = ["loft", "ruled", "point-end", "bezier", "capped", "unsettled"]
origin = "OCCT thrusection/specific/K4"
settled = false
+++

A cone on a teardrop, as a ruled loft from a closed Bézier curve to a single point, the solid closed over the curve by a flat face.

The curve is the cubic Bézier curve in the plane z = 0 with control points (−3, 0, 0), (5, −6, 0), (5, 6, 0) and (−3, 0, 0): its two ends meet at (−3, 0, 0), where it has a corner. It is a teardrop, simple and closed, passing through (3, 0, 0). The point is (1, 0.5, 4), above the inside of the teardrop. The point is joined to every point of the curve by a straight line. The teardrop encloses the area 72/5, so the solid has volume ⅓·(72/5)·4 = 96/5; the area of the conical face has no closed form.

**As a set:**

$$\{\,(1-s)\,(x_0, y_0, 0) + s\,(1, 0.5, 4) : (x_0, y_0) \in R,\ 0 \le s \le 1\,\}$$

where R is the region the teardrop encloses.
