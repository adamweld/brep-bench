+++
title = "A rectangular tube swept along a cubic Bézier, one long edge rounded"
tags = ["fillet", "sweep", "bezier", "freeform"]
settled = false
origin = "OCCT tests/blend/simple/I5, OCCT tests/blend/simple/I6, OCCT tests/blend/simple/I7, OCCT tests/blend/simple/I8"
+++

The path is the cubic Bézier curve in the plane y = 0 with control points (0, 0, 0), (10, 0, 0),
(20, 0, 10) and (20, 0, 20) (x, y, z), which bends gently (its radius of curvature is nowhere less
than 15). With T(t) its unit tangent, N(t) the unit vector in the plane y = 0 square to T and
turned a quarter turn from it towards +z at the start (N(0) = (0, 0, 1)), the tube is every point

path(t) + u·N(t) + v·(0, 1, 0),   0 ≤ t ≤ 1,  −1 ≤ u ≤ 1,  −2 ≤ v ≤ 2,

the rectangle 2 by 4 carried along the path, square to it, its short sides in the plane of the
path. It has four long faces, the ruled surfaces u = ±1 and v = ±2, and two flat ends, the
rectangles at t = 0 and t = 1.

Round the one long edge u = 1, v = 2 (which starts at (0, 2, 1)) to radius 2/5: a ball of that
radius rolling along the edge, touching the two long faces either side of it from inside, from
end face to end face. The other three long edges stay sharp.

The tube and its round have no closed form, so the case is unsettled until two kernels agree.
