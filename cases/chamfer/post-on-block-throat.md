+++
title = "A post on a block, its foot chamfered by the throat"
tags = ["chamfer", "throat", "concave", "cylinder", "planar", "analytic"]
origin = "OCCT tests/bugs/modalg_7/bug28828_8"

[volume]
value = 4304.55481670339
exact = "4000 + 90π (the block and the post) + 2π(r + d/3)·d²/2 (Pappus: the chamfer's triangle d²/2, its centroid d/3 out from the post), r = 3, d = √2"
tex = '4000 + 90\pi + 2\pi\left(r + \tfrac{d}{3}\right)\tfrac{d^2}{2},\quad r = 3,\ d = \sqrt{2}'

[area]
value = 1775.48265636970
exact = "1600 + 60π (the block less the post's foot, the post's side and top) − π((r + d)² − r²) − 2πrd + π(2r + d)·d√2 (the chamfer's cone added, the ring of the block's top and the band of the post it covers taken away), r = 3, d = √2"
tex = '1600 + 60\pi - \pi\left((r + d)^2 - r^2\right) - 2\pi r d + \pi(2r + d)\,d\sqrt{2},\quad r = 3,\ d = \sqrt{2}'
+++

A block from (−10, −10, 0) to (10, 10, 10), with a post standing on its top: the cylinder of
radius 3 about the z axis from z = 10 to z = 20, joined to the block. The post's side meets the
block's top in a concave circular edge, the circle of radius 3 at z = 10.

Chamfer that edge by a throat of 1, as a weld is sized. A concave chamfer adds material: it fills
the right angle between the post and the block's top. In the plane through the z axis, the
chamfer's section is an isosceles right triangle: its two equal legs run from the edge along the
block's top and up the post's side, and its third side, the chamfer's face, lies 1 from the edge.
The legs are d = 1/cos 45° = √2, so the chamfer meets the top on the circle of radius 3 + √2 at
z = 10 and the post's side on the circle of radius 3 at z = 10 + √2. The chamfer is the cone
between those two circles, and the solid is the block and the post joined, with the ring between
them and the cone filled in.

**As a set:**

$$\{\,|x| \le 10,\ |y| \le 10,\ 0 \le z \le 10\,\} \cup \{\,\rho \le 3,\ 10 \le z \le 20\,\} \cup \{\,z \ge 10,\ \rho \ge 3,\ (\rho - 3) + (z - 10) \le \sqrt{2}\,\}$$
