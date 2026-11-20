+++
title = "A cube and a wide frustum standing beside it, their common part"
tags = ["boolean", "common", "general-position", "planar", "cone"]
settled = false
origin = "OCCT tests/boolean/bopcommon_simple/ZH1"
+++

A cube of edge 10, from (0, 0, 0) to (10, 10, 10). A frustum of a cone with its axis the vertical line through (−3, 5), from z = 0 to z = 10, of radius 14 at the bottom and 10 at the top (its radius falls by 2/5 for every unit of rise): its axis lies outside the cube, beside the side face x = 0, and the frustum is as tall as the cube, its ends level with the cube's bottom and top faces. Take the part common to the two.

The cone's base disc covers the whole of the cube's footprint (its farthest corners (10, 0) and (10, 10) are 13.93 from the axis, inside the radius 14), but at the top, radius 10 reaches only x = 7. So the cone's wall cuts across the cube in a strip that slopes from the far side face at the bottom to x = 7 at the top; the cube's far side face x = 10 is cut away above the base. The wall meets the cube's faces y = 0, y = 10 and x = 10 in conic arcs, so the case has no closed form and is unsettled.
