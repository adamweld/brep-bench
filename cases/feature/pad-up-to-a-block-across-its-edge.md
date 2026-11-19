+++
title = "A slanted square pad extruded until it meets a block across its edge"
tags = ["feature", "extrude", "up-to-shape", "union", "planar", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestPad.py testPadToShapeCase"

[volume]
value = 8768.0
exact = "20³ + ∫∫ (10 + |v|) dv dy over |v| ≤ 4, |y| ≤ 4 (the pad's length along d at v across from its middle) = 8000 + 8·96"
tex = '20^3 + \int_{-4}^{4}\!\int_{-4}^{4} \left(10 + |v|\right) dv\,dy = 8000 + 768'

[area]
value = 2789.490332008122
exact = "2400 the block, less the pad's two end rectangles 2·4√2·8, plus its start 8², its two sides square to y 2·96 and its two sides 14 × 8: = 2880 − 64√2"
tex = '2400 - 2\cdot 4\sqrt{2}\cdot 8 + 8^2 + 2\cdot 96 + 2\cdot 14\cdot 8 = 2880 - 64\sqrt{2}'
+++

A block, and a square pad on a slanted plane, extruded until it meets the block. It reaches the
block across one of its edges: half of the pad ends on the block's top and half on its side.

The block is [0, 20] × [−10, 10] × [−20, 0]. Its top is the plane z = 0 and its side the plane
x = 0, and they meet in the edge along the y axis.

The pad: let d = (1, 0, −1)/√2, pointing down at 45° toward +x, and n = (1, 0, 1)/√2. The square
has side 8 and centre (−5√2, 0, 5√2), 10 back along d from the block's edge. It lies in the plane
through its centre square to d, its sides parallel to the y axis and to n. It is extruded along d
until it meets the block (the block as a whole, not one chosen face of it), and joined to the
block.

The line along d through the square's centre runs into the block's edge. The half of the square on
the +n side runs on to the block's top and the other half to its side. So the pad's far end is two
rectangles meeting along the block's edge: |y| ≤ 4, 0 ≤ x ≤ 4√2 on the top z = 0, and |y| ≤ 4,
−4√2 ≤ z ≤ 0 on the side x = 0. The pad is 10 long along d at its middle and 14 at its two sides
parallel to y, so its two sides square to y are pentagons. The pad and the block are one solid.

**As a set:**

$$K \cup \bigl(P \setminus \{\,x \ge 0,\ z \le 0\,\}\bigr), \quad P = \{\,|x + z| \le 4\sqrt{2},\ |y| \le 4,\ x - z \ge -10\sqrt{2}\,\}$$

K is the block, and P the square's extrusion along d without end. Each line of P along d enters
the block where it enters the quarter-space x ≥ 0, z ≤ 0.
