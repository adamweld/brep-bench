+++
title = "A bent tube, one end moved in"
tags = ["edit", "move-face", "sweep", "bezier", "unsettled"]
origin = "OCCT tests/offset/faces_type_i/F7"
settled = false
+++

The bent tube: the solid swept by a disk of radius 1.5 that moves along the cubic Bézier curve with
control points (0, 0, 0), (10, 0, 0), (10, 10, 5) and (20, 10, 5), always square to the curve. At
every point of the curve the disk lies in the plane square to it, centered on it. The curve runs
along +x at both ends, so the tube's end faces are the flat disks in the planes x = 0 and x = 20.
The curve's radius of curvature is everywhere far more than 1.5, so the tube does not overlap
itself.

Then edit the solid directly: move the end in the plane x = 0 by 3 inward along its normal, +x, to
the plane x = 3. The tube is trimmed back to meet it, so that end is now the section of the tube by
x = 3, a flat face slightly out of square to the curve. The other end is untouched.

So the result is the part of the tube with x ≥ 3.

**As a set:**

$$\text{tube} \cap \{\, x \ge 3 \,\}$$
