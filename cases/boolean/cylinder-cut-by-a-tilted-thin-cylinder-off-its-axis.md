+++
title = "A thick cylinder with a tilted thin cylinder cut into its side off-centre"
tags = ["boolean", "cut", "cylinder", "oblique"]
origin = "OCCT tests/bugs/modalg_5/bug25715_1"
settled = false
+++

A thick cylinder of radius 5 about the z axis, from z = 0 to z = 10.

A thin cylinder of radius 1, its axis the line through (1, −7, 7) with direction (0, cos 30°, −sin 30°): it runs from the point (1, −7, 7) for a length of 9, down and inward, to its flat end centred on (1, −7 + 9 cos 30°, 7 − 4.5) = (1, 0.7942, 2.5), the end a disc of radius 1 square to the axis. Its axis passes 1 to the side of the thick cylinder's axis, never meeting it, so it enters the thick cylinder's side wall off-centre, where the wall is at y ≈ −4.9 and z ≈ 5.8, and ends inside it, well clear of the walls and the ends.

Cut the thin cylinder from the thick one: a blind round hole, slanting down into the side. The cut leaves no stray vertices on the curve where the two walls meet.

Neither the volume nor the area has a closed form: the hole's wall meets the thick wall along a saddle curve of two cylinders crossing obliquely. The case is unsettled until two kernels agree.
