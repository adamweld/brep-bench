+++
title = "A cylinder with a conical pocket bored in from its side"
tags = ["boolean", "cut", "cone", "unsettled"]
origin = "OCCT tests/boolean/bopcut_simple/ZK9"
settled = false
+++

A cylinder of radius 5 and height 10 standing on the xy plane, its axis the z axis, and a cone frustum whose axis is the horizontal line through (0, 0, 5) along x: from x = −8, where its radius is 1 and which is outside the cylinder, to x = 2, where its radius is 4 and which is inside it. Cut the frustum away from the cylinder.

The frustum enters through the cylinder's wall on the −x side and stops inside the cylinder: its wide end, the disc of radius 4 in the plane x = 2 (z from 1 to 9, y from −4 to 4, inside the wall), is the flat floor of a conical blind pocket. It has no closed form.
