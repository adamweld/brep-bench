+++
title = "Three quarters of a torus shrunk by 1"
tags = ["offset", "shrink", "torus", "planar"]
origin = "OCCT tests/offset/shape_type_i/C2, C4"
settled = false
+++

Three quarters of the solid torus about the z axis whose tube, of radius 3, runs round the circle
of radius 10 in the plane z = 0: the part whose angle about the z axis, φ, measured from +x toward
+y, lies between 0° and 270°, so everything but the quarter with x ≥ 0 and y ≤ 0. Its ends are
disks of radius 3 in the half-planes y = 0, x ≥ 0 and x = 0, y ≤ 0. Shrink it by 1: the solid of
every point at least 1 from its outside.

The torus moves in to the tube of radius 2 about the same circle, and each end disk moves 1 into
the solid: the one in y = 0 to the plane y = 1, the one in x = 0 to the plane x = −1. Each moved end
is the section of the thinner tube by its plane, an oblique ellipse-like closed curve, not a disk,
and the tube's wall runs up to it. The two ends lie far apart, so nothing else meets.

The section of a torus's tube by a plane parallel to its axis is a spiric curve whose area has no
elementary closed form, so the volume and area are unsettled until two kernels agree.

**As a set:**

$$\{\,(\rho - 10)^2 + z^2 \le 3^2,\ 0^\circ \le \varphi \le 270^\circ \,\} \ominus B_1$$
