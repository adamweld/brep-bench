+++
title = "A post standing out of a block's side, the two straight edges where it leaves the side chamfered with two distances"
tags = ["chamfer", "asymmetric", "concave", "cylinder", "plane", "two-edges"]
origin = "OCCT tests/bugs/modalg_7/bug28828_16"

[volume]
value = 2687.6476090008061
exact = "10·(222.4 + 25π − 25 atan(24/7)): the section times the height 10; the section is the block's rectangle 200, the part of the disc beyond the block's face 25π − (25 atan(4/3) − 12), and two fills, each 26/5 − (25/2) atan(44/117)"
tex = '2224 + 250\pi - 250\arctan\tfrac{24}{7}'

[area]
value = 1255.2375963936779
exact = "twice the section 2(222.4 + 25π − 25 atan(24/7)) plus 10 times its boundary 50 + 2√(13/5) + 5(π + 2 atan(7/24)): 944.8 + 75π + 150 atan(7/24) + 20√(13/5)"
tex = '944.8 + 75\pi + 150\arctan\tfrac{7}{24} + 20\sqrt{\tfrac{13}{5}}'
+++

In the xy plane, take the rectangle from (−10, −10) to (10, 0) and the disc of radius 5 centered at
(0, 3), and join them: a block with a half-buried post standing out of its face y = 0, the disc
crossing that face at (−4, 0) and (4, 0). Extrude the join along +z from z = 0 to z = 10. The two
straight edges (±4, 0, z) where the post's side meets the block's face y = 0 are concave.

Chamfer both with two distances: 1 across the face y = 0, and on the post the straight-line
distance 4/√5 ≈ 1.789 from the edge, in the plane z = const, which is square to it. A concave
chamfer adds material: it fills the groove between the post and the face.

On the right edge the chamfer meets the face at (5, 0, z) and the post at (24/5, 8/5, z), the point
of the circle 4/√5 from (4, 0); the chamfer is the plane through the two lines, which runs the whole
height. The left edge is the mirror image in x = 0. The solid is the extrusion of the section, the
rectangle and the disc with the two small regions between the face, the circle and the chamfer
filled in; its top and bottom faces are flat.

**As a set:**

$$\bigl(\{\,|x| \le 10,\ -10 \le y \le 0\,\} \cup \{\,x^2 + (y-3)^2 \le 25\,\} \cup T_{+} \cup T_{-}\bigr) \times [0, 10]$$

where T₊ is the triangle with corners (4, 0), (5, 0) and (24/5, 8/5), and T₋ its mirror image in x = 0.
