+++
title = "A hollow tube whose section is a ring, screwed round the z axis"
tags = ["sweep", "helix", "screw", "torus", "cylinder", "tube"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestHelix.py::testHelicalTubeCase"
settled = false
+++

A tube whose section is a ring, standing clear of the z axis, screwed round it.

The section is the ring between the circles of radius 2 and radius 1 about the point (8, 0, 2) in the plane y = 0. It is screwed about the z axis: turned right-handed (from +x toward +y) while it rises 6 for every turn, through three whole turns, so that it ends in the plane y = 0 again as the ring about (8, 0, 20).

Each point of the section keeps its distance from the axis. So the tube's outside and its bore are surfaces swept by circles, its two ends are flat rings in the plane y = 0, and its turns, 6 apart while the ring is 4 high, leave gaps of 2 between them. It has no closed form for its area, though its volume is 3π · 2π · 8 · 3 = 144π².

**As a set:**

$$\bigcup_{t \in [0,\,3]} T(6t)\, R_z(360^\circ t)\, Q$$

Q is the ring, T(s) the move up by s along the z axis, and R_z(α) the turn about the z axis by α.
