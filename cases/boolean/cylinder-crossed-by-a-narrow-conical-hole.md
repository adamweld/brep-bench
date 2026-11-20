+++
title = "A cylinder crossed by a conical hole narrower than it"
tags = ["boolean", "cut", "cone", "unsettled"]
origin = "OCCT tests/boolean/bopcut_simple/ZK5"
settled = false
+++

A cylinder of radius 5 and height 10 standing on the xy plane, its axis the z axis, and a cone frustum whose axis is the horizontal line through (0, 0, 5) along x, crossing the cylinder's axis at right angles at mid-height: from x = −7, where its radius is 1, to x = 7, where its radius is 3. Cut the frustum away from the cylinder.

The frustum is longer than the cylinder is wide, at both ends, and its widest radius, 3, is less than both the cylinder's radius and half its height, so it stays clear of the cylinder's end faces and goes in through one side of the cylinder's wall and out through the other: a conical cross hole, 1 to 3 in radius, narrowing toward −x. It has no closed form: the hole's surface meets the wall in a curve of the fourth degree.
