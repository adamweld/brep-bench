+++
title = "A prism on half an ellipse with its curved top edge rounded with a growing radius"
tags = ["fillet", "variable-radius", "elliptic-cylinder"]
origin = "OCCT tests/blend/buildevol/C6, OCCT tests/blend/buildevol/C7"
settled = false
+++

The prism over half an ellipse: the part of the elliptic cylinder x²/20² + y²/12² ≤ 1 with y ≥ 0 and 0 ≤ z ≤ 10, with a flat side in the plane y = 0, the elliptic wall, and a flat top and bottom.

Round the edge where the top (z = 10) meets the elliptic wall, the half ellipse from (20, 0, 10) to (−20, 0, 10), with a radius that depends on where it is along the length: r = 2 − x/20, so 1 where the edge meets the flat side at (20, 0, 10), 2 at the top of the ellipse (0, 12, 10) and 3 at the other end (−20, 0, 10). At each point of the edge the round's section, in the vertical plane through that point square to the edge (the plane that contains the wall's horizontal normal there), is the quarter circle of radius r tangent to the top and to the wall. The edge's curvature radius is smallest, 12²/20 = 7.2, at (0, 12, 10), more than the largest radius, so the round does not fold. At the flat side it meets the plane y = 0 in a quarter circle of radius 1 at one end and of radius 3 at the other.

The round's volume and area have no closed form, so the case is unsettled until two kernels agree.
