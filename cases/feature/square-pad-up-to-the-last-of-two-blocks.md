+++
title = "A square pad extruded through one block and up to the far face of a second"
tags = ["feature", "extrude", "up-to-last", "union", "coplanar", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestPad.py::testPadtoLastCase; src/Mod/PartDesign/PartDesignTests/TestPad.py::testPadToFirstCase"

[volume]
value = 4744.0
exact = "2·8·16² + the pad outside the blocks, 6²·(10 + 8) = 648: 4096 + 648"
tex = '2\cdot 8\cdot 16^2 + 36\cdot 18'

[area]
value = 2408.0
exact = "two blocks 2·1024, less the pad's four contacts with their faces (36 each at the first block's two faces and the second's near face; the pad's end is level with the second's far face and adds nothing there), plus the pad's exposed sides 4·6·10 + 4·6·8 and its start 36: 2048 − 108 + 240 + 192 + 36"
+++

A rectangle extruded along x through one block and up to the far face of a second, joined to both.

Two blocks stand apart on the x axis: the first [10, 18] × [−8, 8] × [−8, 8] and the second
[26, 34] × [−8, 8] × [−8, 8], a gap of 8 between them. The square |y| ≤ 3, |z| ≤ 3 in the plane x = 0 is
extruded along +x up to the last block it meets: through the first block and across the gap, and on
through the second block to its far face, x = 34, where it ends level with that face. The pad and the two
blocks are fused into one solid. The pad runs inside both blocks and ends flush with the second block's
far face.

**As a set:**

$$[10, 18] \times [-8, 8]^2 \ \cup\ [26, 34] \times [-8, 8]^2 \ \cup\ [0, 34] \times [-3, 3]^2$$
