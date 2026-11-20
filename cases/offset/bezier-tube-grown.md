+++
title = "A tube swept along a Bezier S, grown by 1"
tags = ["offset", "grow", "sweep", "freeform", "round"]
origin = "OCCT tests/offset/shape_type_i/C9"
settled = false
+++

The tube swept by a disk of radius 2 along the cubic Bezier curve with control points (0, 0, 0),
(12, 0, 0), (12, 12, 0) and (24, 12, 0): an S in the plane z = 0, of length about 27.73, whose least
radius of curvature, about 12.5, is far more than the tube's radius. At each point of the curve the
disk lies in the plane perpendicular to the curve and is centered on it. The tube's faces are its
side, the surface of every point 2 from the curve, and two flat disks of radius 2 at its ends,
perpendicular to the curve there: in the plane x = 0 at the start and in the plane y = 12 at the
end.

Grow it by 1: the solid of every point within 1 of the tube.

The side moves out to the surface of every point 3 from the curve. Each flat end disk moves 1 out
along the curve's tangent there, and the rim where it met the side, a convex edge, becomes a quarter
of a torus of tube radius 1 about the rim's circle. The offset of a freeform sweep has no closed
form, so the case is unsettled until two kernels agree.
