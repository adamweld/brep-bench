+++
title = "A cube mirrored, repeated in a row and turned four ways"
tags = ["pattern", "mirror", "linear", "polar", "union", "face-contact", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestMultiTransform.py::testMultiTransform"

[volume]
value = 20000.0
exact = "20 distinct cubes of 10³ (the union of the cells of the grid of 10 that the transforms reach)"
tex = '20\cdot 10^3'

[area]
value = 8000.0
exact = "top and bottom 2·20·100, and the outline of the 20 cells, 40 cell edges of 10 by height 10: 4000 + 4000"
tex = '2\cdot 20\cdot 10^2 + 40\cdot 10\cdot 10'
+++

A cube mirrored, repeated in a row and turned about an axis, all fused into one solid.

Start with the cube [10, 20] × [0, 10] × [0, 10]. Mirror it across the plane x = 0, which adds
[−20, −10] × [0, 10] × [0, 10]. Repeat that pair along +y at spacing 10, three in all (the pair as it is and
two copies, one and two cubes' widths further along), so the pair is now two rows
[10, 20] × [0, 30] × [0, 10] and [−20, −10] × [0, 30] × [0, 10]. Finally turn the whole thing about the z
axis, four copies at 0°, 90°, 180° and 270°. Fuse everything into one solid. The copies overlap where the
turns bring the rows across one another, and touch face to face elsewhere; no two cubes meet along only an
edge.

So the solid is twenty distinct cubes of side 10 on the grid of 10, in one layer.

**As a set:**

$$\bigcup_{k=0}^{3} R_z(90^\circ k)\Bigl(\bigl([10, 20] \cup [-20, -10]\bigr) \times [0, 30]\Bigr) \times [0, 10]$$
