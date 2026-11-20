+++
title = "A cylinder crossed by a conical hole wider than the cylinder is tall"
tags = ["boolean", "cut", "cone", "unsettled"]
origin = "OCCT tests/boolean/bopcut_simple/ZK7"
settled = false
+++

A cylinder of radius 5 and height 10 standing on the xy plane, its axis the z axis, and a cone frustum whose axis is the horizontal line through (0, 0, 5) along x, crossing the cylinder's axis at right angles at mid-height: from x = −7, where its radius is 2, to x = 7, where its radius is 7. Its radius is 5 at x = 1.4 and more beyond, so on that side the hole is wider than the cylinder is tall and overshoots the cylinder's top and bottom faces: the cone's surface cuts both of them. Cut the frustum away from the cylinder. It has no closed form.
