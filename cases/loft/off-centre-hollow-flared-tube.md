+++
title = "A flaring hollow tube with its inner circle off-centre at the foot"
tags = ["loft", "ruled", "hollow", "oblique-cone", "non-concentric"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestLoft.py::testIssue6130NestedCirclesIgnoreCreationOrder"
settled = false
+++

A hollow tube flaring upward whose inside is slightly off-centre at its foot: a ruled loft between a
pair of circles that are not concentric and a larger concentric pair.

At z = 0 the outer circle has radius 20 and centre (0, 0), and the inner circle radius 10 and centre
(2, 0), a little off the outer one's. At z = 15 the outer circle has radius 30 and the inner one radius 18,
both centred at (0, 0). The loft is **ruled**: the outer circle at the foot is joined to the outer circle at
the top, and the inner to the inner, the point at angle φ about one circle's centre to the point at angle φ
about the other's, by a straight line. So the outer wall is a cone frustum of revolution and the inner wall
is a slanting (oblique) circular cone; the solid is what lies between them, with the flat ring between the
two circles at each end.
