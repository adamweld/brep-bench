+++
title = "A cube and a wide frustum centred on a side face, spilling past its edges, their common part"
tags = ["boolean", "common", "general-position", "planar", "cone"]
settled = false
origin = "OCCT tests/boolean/bopcommon_simple/ZG1"
+++

A cube of edge 10: x from −5 to 5, y from 0 to 10, z from 0 to 10. A frustum of a cone with its axis the vertical line through (0, 0), from z = 0 to z = 10, of radius 7 at the bottom and 3 at the top (its radius falls by 2/5 for every unit of rise). Its axis lies in the cube's side face y = 0, at the middle of it, and the frustum is as tall as the cube, its ends level with the cube's bottom and top faces. Take the part common to the two.

The base circle's radius 7 is more than half the face's width, 5, so the lower part of the cone spills past the faces x = −5 and x = 5 and is cut by them; above z = 5 (where the radius falls to 5) the cone lies within the cube's width. The cone's wall meets the faces x = ±5 in hyperbola arcs, so the case has no closed form and is unsettled. The half with y ≥ 0 is the part inside the cube; the cone does not reach the cube's far face y = 10.
