+++
title = "A leaning wedge on a plate with rounded top edges, shelled inward through the plate's underside"
tags = ["shell", "union", "fillet", "planar", "cylinder", "sphere", "concave"]
settled = false
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestThickness.py::testCase5829ThicknessOnRotatedFillet"
+++

A plate, the box from the origin to (10, 8, 3), with the four edges of its top face (z = 3) rounded
to a radius of 1, each round tangent to the top and to the side it runs along, and meeting its
neighbours at the plate's corners. Standing on its flat top is a leaning wedge: the prism along y,
from y = 2 to y = 6, of the triangle in the xz plane with corners (3, 3), (7, 3) and (8, 9). Its
foot lies on the plate's top flat (which spans 1 ≤ x ≤ 9, 1 ≤ y ≤ 7), and its apex leans out beyond
its foot, over x = 8. The wedge and the plate are united into one solid.

The solid is shelled inward to a wall 0.5 thick, open at the plate's underside, its bottom face
z = 0: the bottom face is removed, and the wall is every point of the solid within 0.5 of its
other faces. The wall's inside is the surface 0.5 inside those faces: planes moved in, the rounds'
cylinders and corner patches moved in to radius 0.5, and at the concave edges where the wedge's
sides meet the plate's top, a round of radius 0.5 about the edge. The inside meets the opening in
the plane z = 0, which is the wall's rim. The wedge's apex is hollow, its cavity a sharp edge where
its two moved sides meet.

It has no closed form for its volume or its area: the answer is the solid's volume less its cavity,
the cavity being the points of the solid at least 0.5 from every point of its surface but the
bottom face.

**As a set:**

$$S \setminus \{\, p \in S : d(p,\ \partial S \setminus \{z = 0\}) \ge 0.5 \,\}$$

S is the plate united with the wedge.
