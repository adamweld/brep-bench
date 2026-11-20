+++
title = "An ellipse swept along a twisted cubic Bézier curve in its Frenet frame"
tags = ["sweep", "space-path", "bezier", "torsion", "unsettled"]
origin = "OCCT pipe/standard/B7, OCCT pipe/standard/B8"
settled = false
+++

An ellipse swept along a cubic Bézier curve that does not lie in a plane, carried in the curve's Frenet frame.

The curve's four control points are (0, 0, 0), (12, 16, 0), (26, 8, 0) and (24, 0, 16): it leaves
the origin along (3, 4, 0), swings round to the right and rises out of the xy plane to end at
(24, 0, 16). Its torsion never vanishes and its radius of curvature is least, 10.28, a little past
the middle.

The section is an ellipse centered on the curve and square to it, with semi-axis 6 along the
curve's principal normal and semi-axis 3 along its binormal (the Frenet frame: the principal
normal is the direction of the curve's second derivative with its tangential part removed, the
binormal the tangent crossed with it), at every point. The ends are flat, square to the curve at
its two ends. The wall does not meet itself.

The volume is πab·L = 18π·40 = 720π whatever the frame, since the section's center is on the
curve and the curve's length is 40; the wall's area has no closed form, as how the ellipse turns
about the curve decides it.
