+++
title = "An ellipse swept along a parabola arc, its minor axis in the path's plane"
tags = ["sweep", "planar-path", "conic", "unsettled"]
origin = "OCCT pipe/standard/B4, OCCT pipe/standard/B3, OCCT pipe/standard/B6"
settled = false
+++

An ellipse swept along an arc of a parabola, carried so that it keeps its orientation to the curve.

The path lies in the xy plane: the curve y = x²/16, from the origin to (16, 16, 0); its radius of
curvature is least at the origin, 8, and it bends toward +y.

The section is an ellipse centered on the path and square to it, with semi-axis 3 along the path's
principal normal (the direction in the xy plane toward the side the path bends to, which at the
origin is +y) and semi-axis 5 along the normal to the xy plane. This stays so all along the path:
the section is carried in the path's Frenet frame, tangent, principal normal and binormal. The
ends are flat, square to the path: in the plane x = 0, and in the plane through (16, 16, 0) square
to the path there. The wall does not meet itself (the section's reach toward the bend, 3, is under
the radius of curvature, 8).

The volume is πab·L with a = 5, b = 3 and L = 8√5 + 4·asinh 2 the path's length, as the section's
center is on the path; the wall's area has no closed form.
