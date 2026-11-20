+++
title = "A pipe whose circular section swells along a quadratic Bezier curve"
tags = ["sweep", "planar-path", "bezier-path", "variable-section", "unsettled"]
origin = "OCCT pipe/bugs/bug30003"
settled = false
+++

A pipe of circular section whose radius grows steadily along the path, swept along a short
quadratic Bezier curve.

The path is the quadratic Bezier curve C(t), 0 ≤ t ≤ 1, of control points (0, 0, 0), (8, 10, 0)
and (20, 4, 0), in the xy plane: it leaves the origin along (4, 5, 0) and bends to the right, to
arrive at (20, 4, 0) heading along (12, −6, 0).

The section at C(t) is the disk centered on C(t), in the plane square to the path there, of
radius 1.5 + 2t: radius 1.5 at the start, 3.5 at the end, growing in proportion to the curve's
parameter t (not to its length). The path's radius of curvature is at least 25, well above the
pipe's radius, so the pipe never folds. The two ends of the pipe are flat disks, one of radius
1.5 square to the path at the origin and one of radius 3.5 square to the path at (20, 4, 0).
