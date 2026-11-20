+++
title = "A cube with an edge chamfered and the chamfer's two long edges rounded"
tags = ["fillet", "chamfer", "analytic", "planar", "cylinder"]
origin = "OCCT tests/bugs/moddata_1/bug203"

[volume]
value = 978.2788415460503
exact = "10·(100 − 2 − 2r²(tan(π/8) − π/8)), r = 2"
tex = '10\left(98 - 2r^2\left(\tan\tfrac{\pi}{8} - \tfrac{\pi}{8}\right)\right),\quad r = 2'

[area]
value = 582.2188811027223
exact = "2·A + 10·P, A = 98 − 2r²(tan(π/8) − π/8), P = 36 + 2√2 + 2(rπ/4 − 2r·tan(π/8)), r = 2"
tex = '2A + 10P,\quad P = 36 + 2\sqrt2 + 2\left(\tfrac{\pi r}{4} - 2r\tan\tfrac{\pi}{8}\right)'
+++

A cube of side 10 between the origin and (10, 10, 10). Chamfer the edge where its faces x = 10 and
y = 10 meet by a distance of 2 on both faces: a flat face, running the cube's whole height, between
the section points (8, 10) and (10, 8). Then round to radius 2 the two long edges of that chamfer
face, the one where it meets the face y = 10 and the one where it meets the face x = 10.

At each of them two faces meet at an interior angle of 135°, so a round of radius 2 touches each
face 2·tan(22.5°) ≈ 0.83 from the edge, and the two rounds leave a strip of the chamfer face
between them (its width 2√2 ≈ 2.83 is more than 2 × 0.83). Both rounds run straight through
the height of the cube and end square on its top and bottom.

The solid is a prism along z on a section: the square from (0, 0) to (10, 10), its corner cut by
the chamfer, and the two corners the chamfer makes with the square's sides rounded by arcs of
radius 2.
