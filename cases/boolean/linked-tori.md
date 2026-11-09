+++
title = "Two separate rings linked through each other"
tags = ["boolean", "union", "analytic", "torus", "lumps", "genus"]

[volume]
value = 29.608813203268074
exact = "2·2π²Rr² = 3π², R = 3, r = 1/2"
tex = '2\cdot 2\pi^2\cdot 3\cdot\left(\tfrac12\right)^2 = 3\pi^2'

[area]
value = 118.4352528130723
exact = "2·4π²Rr = 12π², R = 3, r = 1/2"
tex = '2\cdot 4\pi^2\cdot 3\cdot\tfrac12 = 12\pi^2'
+++

Two solid tori, each with middle-circle radius 3 and tube radius 0.5:

- the first centered at the origin, about the z axis, with its middle circle in the xy plane;
- the second centered at (3, 0, 0), about a line parallel to y, with its middle circle in the
  plane y = 0.

Unite them, keeping both bodies. They do not touch: the least distance between their middle
circles is 3, so their surfaces remain 2 apart. They are linked once. The second middle circle
crosses z = 0 at (0, 0, 0), inside the disk spanned by the first middle circle, and (6, 0, 0),
outside it. Neither ring lies inside the material or a closed cavity of the other.

The volume and area are the sums for two complete tori. Their specified placement matters to
this case's geometry, although the suite's scalar measures and mesh-closure checks alone cannot
distinguish this link from two unlinked rings of the same dimensions.
