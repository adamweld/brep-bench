+++
title = "A bent tube, both its ends moved out"
tags = ["edit", "move-face", "sweep", "bezier", "unsettled"]
origin = "OCCT tests/offset/faces_type_i/F8"
settled = false
+++

The bent tube: the solid swept by a disk of radius 1.5 that moves along the cubic Bézier curve with
control points (0, 0, 0), (10, 0, 0), (10, 10, 5) and (20, 10, 5), always square to the curve. At
every point of the curve the disk lies in the plane square to it, centered on it. The curve runs
along +x at both ends, so the tube's end faces are the flat disks in the planes x = 0 and x = 20.
The curve's radius of curvature is everywhere far more than 1.5, so the tube does not overlap
itself.

Then edit the solid directly: move the end in the plane x = 0 by 2 outward along its normal, −x, to
the plane x = −2, and the end in the plane x = 20 by 2 outward along its normal, +x, to the plane x
= 22. The tube runs on to meet them, along the same Bézier curve continued past its ends (the same
cubic polynomial, for parameter below 0 and above 1), swept by the same disk of radius 1.5 square to
it, so each end is now the section of that extended tube by its plane, a flat face slightly out of
square to the curve.

So the result is the tube extended along its curve, and cut by the planes x = −2 and x = 22.

**As a set:**

$$\text{extended tube} \cap \{\, -2 \le x \le 22 \,\}$$
