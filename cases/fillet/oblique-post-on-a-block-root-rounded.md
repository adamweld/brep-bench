+++
title = "A leaning cylinder standing on a block, its root rounded"
tags = ["fillet", "concave", "cylinder", "oblique"]
origin = "OCCT tests/blend/bfuseblend/B1"
settled = false
+++

A block from (−30, −30, 0) to (30, 30, 10), and standing on its top face a cylinder of radius 5 leaning 30° from the vertical toward +x: its axis is the line through (0, 0, 10) in the direction (sin 30°, 0, cos 30°). The cylinder runs along its axis from the disk 5 below the top face's level (axial distance −5, centre (−2.5, 0, 5.67), which lies wholly inside the block) to a flat end square to the axis at axial distance 20, centre (10, 0, 27.32). Its part inside the block adds nothing: the two are one solid.

The cylinder meets the top face along an ellipse in the plane z = 10, centre (0, 0, 10), semi-axes 5 along y and 5 / cos 30° = 5.7735 along x. Round it to radius 1.5. The round is concave, a ball of radius 1.5 rolling round the junction touching the top face and the cylinder. The angle between the two faces changes all the way round, from 60° on the side toward which the post leans (the empty wedge between the block's top and the cylinder's wall is 60° there) to 120° on the opposite side, so the round's section changes all the way round and it is no torus.

Its volume and area have no closed form, so the case is unsettled until two kernels agree.
