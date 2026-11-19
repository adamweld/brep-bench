+++
title = "A half cone cut by a thin cylinder across its slanted wall"
tags = ["boolean", "cut", "cone", "cylinder"]
origin = "OCCT mesh/bug25157"
settled = false
+++

A cone of base radius 10 and height 20: its base the disc of radius 10 about the z axis at z = 0, its apex at (0, 0, 20). Keep the half of it with y ≥ 0. Cut from that half the cylinder of radius 1 about the vertical line through (x, y) = (4, 3), from z = −5 to z = 25, which runs clear through the cone from below its base to above its apex. The cylinder crosses the slanted wall of the cone, where the cone's radius is between 4 and 6, at heights between 8 and 12; it does not touch the flat face at y = 0.
