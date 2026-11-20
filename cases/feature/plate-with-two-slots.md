+++
title = "A plate with two slots, extruded from its outline"
tags = ["feature", "extrude", "slot", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestPad.py::testBoxCase; src/Mod/PartDesign/PartDesignTests/TestPad.py::testStartOffsetForTwoSidedAndSymmetricPad; src/Mod/PartDesign/PartDesignTests/TestPad.py::testPadTwoDimensionsCase"

[volume]
value = 10916.81421051076
exact = "6(2400 − 2(30·8 + 16π)) = 6(1920 − 32π)"
tex = '6\left(1920 - 32\pi\right)'

[area]
value = 5860.530964914873
exact = "two faces 2(1920 − 32π) and the walls 6 thick round the outline 200 and the two slots 2(2·30 + 2π·4): 5760 + 32π"
tex = '5760 + 32\pi'
+++

A flat plate with two slots cut through it, extruded from its outline.

The outline is drawn in the plane z = 0: the rectangle [0, 60] × [0, 40] with two slot-shaped holes in it.
A slot is a stadium: a rectangle with a half circle on each end. The first has its end centres at (15, 12)
and (45, 12), the second at (15, 28) and (45, 28), each of radius 4. So each is 38 long and 8 wide, and
the wall between them is 8 thick. The outline is extruded 2 up and 4 down from its plane, giving a plate
6 thick, [0, 60] × [0, 40] × [−4, 2] with the two slots through it.

**As a set:**

$$\Bigl([0, 60] \times [0, 40] \setminus \bigl(\operatorname{conv}(B_4(15, 12) \cup B_4(45, 12)) \cup \operatorname{conv}(B_4(15, 28) \cup B_4(45, 28))\bigr)\Bigr) \times [-4, 2]$$

where the discs B are in the (x, y) plane.
