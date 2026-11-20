+++
title = "A post on a slanted face, the elliptical edge between them chamfered by equal distances"
tags = ["chamfer", "concave", "cylinder", "plane", "ellipse"]
origin = "OCCT tests/bugs/modalg_7/bug28828_2"
settled = false
+++

A block from (−20, −20, −20) to (20, 20, 0), turned by 20° about the y axis, right-handed about +y
(from +z toward +x), so that its top face passes through the origin, leaning down toward +x. Join it
to a post, the cylinder of radius 4 on the z axis from z = −5 to z = 12. The post's side meets the
slanted top face in an ellipse, a concave edge whose dihedral angle varies round it, sharper on one
side of the post than on the other.

Chamfer that edge with equal distances, 1 on each face. At every point P of the edge, take the plane
through P square to the edge there. In that plane the chamfer meets the top face at the point of the
face at straight-line distance 1 from P, on the side away from the post, and meets the post at the
point of the post's side at straight-line distance 1 from P, the upper one of the two such points
(the other lies down inside the block). A concave chamfer adds material: the chamfer's face is the surface ruled between the two curves those points
trace, and it fills the groove between the post and the face.

The chamfer's face is not a cone or a plane, and no closed form for the volume is given.

**As a set:**

The block and the post joined, with the region between the slanted face, the post and that ruled
surface filled in.
