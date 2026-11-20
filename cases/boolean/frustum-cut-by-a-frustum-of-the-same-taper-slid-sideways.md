+++
title = "A cone frustum cut by a frustum of the same taper on a parallel axis"
tags = ["boolean", "cut", "cone", "unsettled"]
origin = "OCCT tests/boolean/bopcut_simple/ZN2"
settled = false
+++

A cone frustum on the z axis from z = 0 to z = 8: radius 6 at its base, radius 3 at its top. And a second frustum on the vertical line through (4, 0), also from z = 0 to z = 8: radius 5 at its base, radius 2 at its top. Both lose 3/8 of a unit of radius per unit of height, so their sides are the same cone moved sideways, and their bases lie in one plane and their tops in another. The second overlaps the first partly (its base disc spans x from −1 to 9, the first's from −6 to 6). Cut the second frustum away from the first. It has no closed form.
