+++
title = "A pillow block mirrored from its half"
tags = ["pattern", "mirror", "union", "coincident", "planar", "cylinder", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestMirrored.py::testMirroredSketchCase; tests/src/Mod/Part/App/FeatureMirroring.cpp::testXMirror"

[volume]
value = 44568.85046025214
exact = "2(40·30·12 + 20·30·22 − (π·10²/2)·30 − π·4²·12) = 2(27600 − 1692π) = 55200 − 3384π: twice the half, the base and the housing above it, less half the bore and one bolt hole"
tex = '2\left(40\cdot 30\cdot 12 + 20\cdot 30\cdot 22 - \tfrac{\pi\cdot 10^2}{2}\cdot 30 - \pi\cdot 4^2\cdot 12\right) = 55200 - 3384\pi'

[area]
value = 12178.760921095411
exact = "(2400 − 32π) + (1200 − 32π) + 720 + 1320 + 1200 + 2(1840 − 100π) + 2π·10·30 + 2·2π·4·12 = 10520 + 528π: bottom, the base's top beside the housing, the base's ends, the housing's sides and top, front and back each less the bore's circle, the bore and the two holes; the faces in the plane x = 0 are gone"
tex = '(2400 - 32\pi) + (1200 - 32\pi) + 720 + 1320 + 1200 + 2\left(1840 - 100\pi\right) + 2\pi\cdot 10\cdot 30 + 2\cdot 2\pi\cdot 4\cdot 12 = 10520 + 528\pi'
+++

A pillow block, a bearing housing on a base with two bolt holes, made as one half mirrored across
its middle plane x = 0 and joined to its mirror image.

The half, all of it in x ≥ 0:

- the base, the box from (0, −15, 0) to (40, 15, 12);
- the housing, the box from (0, −15, 0) to (20, 15, 34), joined to the base;
- half the bore: the half-cylinder of radius 10 about the line x = 0, z = 20 (running along y)
  taken away, from y = −15 to y = 15, so the half has a half-round groove in its face x = 0 whose
  deepest point is at x = 10;
- a bolt hole of radius 4 on the vertical axis through (30, 0), right through the base from z = 0
  to z = 12.

Mirror the half in the plane x = 0 and join the two. The halves meet face to face over the whole of
that plane, where their faces lie on each other and are no longer faces of the solid: the result
is one solid, symmetric about x = 0, with a full bore of radius 10 through the housing along y,
centred at height 20 (so it comes within 4 of the housing's top at z = 34 and 10 of the bottom),
and a bolt hole
through the base on each side, at x = ±30.

**As a set:** with K the half described,

$$K \cup M K, \quad M(x, y, z) = (-x, y, z)$$
