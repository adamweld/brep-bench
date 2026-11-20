+++
title = "A slab with a hollow stadium-shaped loft cut through it"
tags = ["loft", "ruled", "hollow", "subtractive", "stadium", "oblique-cone"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestLoft.py::testCapsuleSubtractiveLoftIgnoresCreationOrder; src/Mod/PartDesign/PartDesignTests/TestLoft.py::testCapsuleAdditiveLoftIgnoresCreationOrder; src/Mod/PartDesign/PartDesignTests/TestLoft.py::testIssue6130SubtractiveLoftIgnoresCreationOrder; src/Mod/PartDesign/PartDesignTests/TestLoft.py::testSimpleSubtractiveLoftCase"
settled = false
+++

A slab with a hollow stadium-shaped loft cut out of it right through its thickness.

The slab is [−40, 40] × [−30, 30] × [0, 12]. A stadium is a rectangle with a half circle on each end. The
cutting tool is the loft between two pairs of nested stadiums, one pair in the plane z = 0 (the slab's
bottom) and the other in z = 12 (its top), all with their centreline on the x axis. At z = 0: the outer
stadium has its end centres at (−10, 0) and (10, 0) and radius 8, the inner one the same end centres and
radius 4. At z = 12: the outer has its end centres at (−14, 0) and (14, 0) and radius 12, the inner the same
centres and radius 7. The loft is **ruled** and joins outer to outer and inner to inner, each straight side
to its matching straight side and each end arc to its matching arc (the point at angle φ about an end's
centre to the point at angle φ about the matching end's), by straight lines. So the straight sides are flat
(the planes y = ±(8 + z/3) outer, y = ±(4 + z/4) inner) and each end is a slanting cone. The tool is the
solid between the outer and the inner surfaces, flush with the slab's top and bottom; the slab less it
leaves the part of the slab outside the loft and, separately, the core inside the inner surface, two solids.
