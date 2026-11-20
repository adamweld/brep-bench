+++
title = "A round tube swept along a cubic Bézier, its far end edge rounded"
tags = ["fillet", "sweep", "bezier", "pipe", "freeform"]
settled = false
origin = "OCCT tests/blend/simple/J9"
+++

The path is the cubic Bézier curve with control points (0, 0, 0), (10, 0, 0), (20, 0, 10) and
(20, 0, 20) (its radius of curvature is nowhere less than 15). The tube is the solid a disc of
radius 3/2 sweeps as its center runs along the path from the start to the far end, the disc always
square to the path's tangent: a pipe of radius 3/2 with flat discs for ends, one at (0, 0, 0)
square to the x axis and one at (20, 0, 20) square to the z axis.

Round the circular edge around the far end disc, where the disc meets the pipe's side, to radius
1/2. The round is the blend a ball of radius 1/2 sweeps as it rolls round the edge, touching the
disc and the pipe's side from inside. The pipe's other end stays sharp.

The pipe is not a cylinder where it bends, so the round is no torus, and the case is unsettled
until two kernels agree.
