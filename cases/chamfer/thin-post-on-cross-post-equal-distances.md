+++
title = "A thin post standing on a cross post, the saddle edge between them chamfered by equal distances"
tags = ["chamfer", "concave", "cylinder", "saddle"]
origin = "OCCT tests/bugs/modalg_7/bug28828_4"
settled = false
+++

A thick post, the cylinder of radius 6 whose axis is the x axis, from x = −15 to x = 15; joined to a
thin post, the cylinder of radius 3 on the z axis from z = 0 to z = 12, whose base plane z = 0
contains the thick post's axis. The thin post leaves the thick one over its top, where its side meets
the thick post's in a closed saddle-shaped curve (3 cos φ, 3 sin φ, √(36 − 9 sin² φ)), a concave edge
all round; it is highest, at z = 6, where it crosses the plane y = 0, and lowest, at z = √27, where it
crosses x = 0.

Chamfer that edge with equal distances, 1 on each post. At every point of the edge the distance is
measured, on each post, in a straight line from the edge point in the plane square to the edge there:
the chamfer meets each post at the point of its side 1 from the edge, on the side away from the other
post. A concave chamfer adds material: the chamfer's face is the surface ruled between the two curves
those points trace, and it fills the groove between the posts.

The chamfer's face is not a cone or a plane, and no closed form for the volume is given.

**As a set:**

The two posts joined, with the region between their sides and that ruled surface filled in.
