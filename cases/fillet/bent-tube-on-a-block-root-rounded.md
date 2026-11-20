+++
title = "A tube bent along a Bézier curve standing on a block, its root rounded"
tags = ["fillet", "concave", "bezier", "pipe"]
origin = "OCCT tests/blend/bfuseblend/B2"
settled = false
+++

A block from (−30, −30, 0) to (30, 30, 10), and standing on its top face a bent tube: the solid swept by a disk of radius 3 whose centre runs along the cubic Bézier curve with control points (−4, 0, 4), (−1, 0, 14), (8, 0, 18), (20, 0, 32), the disk held square to the curve (in the plane through its centre square to the curve's tangent) from one end of the curve to the other. The curve stays in the plane y = 0 and its smallest radius of curvature, about 20.8, is more than the tube's radius, so the sweep does not fold. The tube's ends are flat disks, the first at (−4, 0, 4), wholly inside the block (the curve leaves it going up and 17° toward +x), the last at (20, 0, 32). The part of the tube inside the block adds nothing: the two are one solid.

The curve crosses the top face's plane z = 10 near (−1.11, 0, 10), leaning 34° from the vertical, so the tube meets the top face along a closed curve close to an ellipse, whose angle to the face changes all the way round. Round that curve to radius 1. The round is a ball of radius 1 rolling round the junction, concave, touching the top face and the tube.

Its volume and area have no closed form, so the case is unsettled until two kernels agree.
