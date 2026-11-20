+++
title = "A tube swept along a Bezier S, shrunk by 1"
tags = ["offset", "shrink", "sweep", "freeform"]
origin = "OCCT tests/offset/shape_type_i/D1"
settled = false
+++

The tube swept by a disk of radius 2 along the cubic Bezier curve with control points (0, 0, 0),
(12, 0, 0), (12, 12, 0) and (24, 12, 0): an S in the plane z = 0, of length about 27.73, whose least
radius of curvature, about 12.5, is far more than the tube's radius. At each point of the curve the
disk lies in the plane perpendicular to the curve and is centered on it. The tube's faces are its
side, the surface of every point 2 from the curve, and two flat disks of radius 2 at its ends,
perpendicular to the curve there: in the plane x = 0 at the start and in the plane y = 12 at the
end.

Shrink it by 1: the solid of every point at least 1 from the tube's outside.

The side moves in to the surface of every point 1 from the curve, and each flat end disk moves 1 in
along the curve's tangent there; the result is the thin tube of radius 1 about the same curve, cut
by the two moved planes, each perpendicular to the curve's tangent at its end and 1 inside it. The
sweep has no closed form, so the case is unsettled until two kernels agree.
