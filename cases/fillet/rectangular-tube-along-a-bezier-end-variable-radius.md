+++
title = "A rectangular tube along a cubic Bézier with its start rim rounded with a radius that varies round it"
tags = ["fillet", "variable-radius", "sweep", "bezier", "freeform", "crease", "closed-edge", "unsettled"]
origin = "OCCT tests/blend/buildevol/E2, OCCT tests/blend/buildevol/E3"
settled = false
+++

The path is the cubic Bézier curve in the plane y = 0 with control points (0, 0, 0), (10, 0, 0),
(20, 0, 10) and (20, 0, 20) (x, y, z), which bends gently (its radius of curvature is nowhere less
than 15). With T(t) its unit tangent, N(t) the unit vector in the plane y = 0 square to T and
turned a quarter turn from it towards +z at the start (N(0) = (0, 0, 1)), the tube is every point

path(t) + u·N(t) + v·(0, 1, 0),   0 ≤ t ≤ 1,  −1 ≤ u ≤ 1,  −2 ≤ v ≤ 2,

the rectangle 2 by 4 carried along the path, square to it, its short sides in the plane of the
path. It has four long faces, the ruled surfaces u = ±1 and the planes v = ±2, and two flat ends, the
rectangles at t = 0 and t = 1.

Round the four edges of the start end, the rectangle in the plane x = 0 with −2 ≤ y ≤ 2 and
−1 ≤ z ≤ 1 (so u = z and v = y there), where it meets the four long faces. The radius varies round
the rim with the point on it: r = 2/5 + z/20 + y/40. So r is 1/2 at the corner (y, z) = (2, 1), 3/10
at (−2, −1) and 2/5 at the other two corners, and goes evenly along each edge between its corners.
At each point of an edge the round's section, in the plane through that point square to the edge, is
the circular arc of radius r that lies inside the tube and touches the end face and the long face
there, each in its trace in that plane (for the edges on the planes y = ±2 both traces are straight
lines and the section is a quarter circle; for the edges on the faces z = ±1 the long face's trace is
slightly curved, the offset of the path).

The tube's four long edges, where the long faces meet, stay sharp. So at each corner of the rim two
rounds meet a sharp long edge, and do not close the corner with a patch of their own: each round runs
the whole length of its edge, neither is shortened by the other, and where they cross they meet in a
crease, the curve where the two surfaces intersect (the two radii are equal there). The far end stays
sharp.

The tube and its round have no closed form, so the case is unsettled until two kernels agree.
