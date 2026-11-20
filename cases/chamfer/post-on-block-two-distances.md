+++
title = "A post on a block, the concave edge between them chamfered with two distances"
tags = ["chamfer", "asymmetric", "concave", "cylinder", "plane"]
origin = "OCCT tests/bugs/modalg_7/bug28828_9"

[volume]
value = 4344.0043955680824
exact = "4000 + 96π (the block and the post above it) + 2π·(9/2)·(3/2) (Pappus: the triangle the chamfer fills, area 3/2, its corners' mean distance from the axis 9/2) = 4000 + 219π/2"
tex = '4000 + \tfrac{219\pi}{2}'

[area]
value = 1730.3760951239764
exact = "(1600 − 121π/4) the block less the disc the chamfer covers + 32π the post above the chamfer + 16π its top + (19π/2)(5/2) the chamfer's cone = 1600 + 83π/2"
tex = '1600 + \tfrac{83\pi}{2}'
+++

A block from (−10, −10, 0) to (10, 10, 10), joined to a post, the cylinder of radius 4 on the z axis
from z = 0 to z = 16. The post's side meets the block's top face in the circle of radius 4 at
z = 10, a concave edge. Chamfer that edge with two distances: 3/2 across the block's top face and 2
up the post. A concave chamfer adds material: it fills the groove between the post and the face.

The chamfer meets the top face on the circle of radius 11/2 at z = 10 and the post on the circle of
radius 4 at z = 12. The chamfer is the cone between those two circles; the solid is the block and the
post joined, with the ring between them and the cone filled in.

**As a set:**

$$\{\,|x|,|y| \le 10,\ 0 \le z \le 10\,\} \cup \{\,\rho \le 4,\ 0 \le z \le 16\,\} \cup \{\,\rho \ge 4,\ z \ge 10,\ \tfrac{\rho - 4}{3/2} + \tfrac{z - 10}{2} \le 1\,\}$$
