+++
title = "A prism on a cubic Bézier and its chord, the curved top edge and the two upright edges rounded, creased at the corners"
tags = ["fillet", "corner", "bezier", "nurbs", "planar", "crease", "unsettled"]
origin = "OCCT tests/blend/simple/G4, OCCT tests/blend/simple/G6"
settled = false
+++

In the plane z = 0, the cubic Bézier curve C with control points (0, 0), (3, 8), (9, 9), (12, 0),
from (0, 0) to (12, 0), is closed by the straight chord from (12, 0) back to (0, 0). The region D they
bound, with the curve above the chord, is extruded to z = 5: a prism with a flat bottom and top, a flat
upright side on the chord (the plane y = 0), and a curved wall of the Bézier cylinder standing on C. The
curve meets the chord at slanting angles (69.4° at (0, 0), 71.6° at (12, 0)), and its radius of
curvature is nowhere less than 3.5.

Round three edges: the one edge where the curved wall meets the top, the curve C at z = 5, to radius 1,
and the two upright edges where the wall meets the flat side, at (0, 0) and (12, 0), to radius 3/2.

- The top round is the blend a ball of radius 1 sweeps rolling along C at z = 5, touching the top and the
  curved wall from inside. Its ends are cut by the plane y = 0 obliquely.
- Each upright round is a vertical surface, a circular arc in plan of radius 3/2 tangent to the flat
  side and to the wall, running from the bottom to the top.

The top edge of the flat side, along y = 0 at z = 5, is not rounded. So at each top corner, (0, 0, 5)
and (12, 0, 5), the top round and the upright round do not close the corner with a patch of their own:
each runs the whole length of its edge, neither is shortened by the other, and where they cross they
meet in a crease, the curve where the two surfaces intersect. The solid is the prism less what each
round removes.

**As a set:** with d the distance in the plane and D′ = (D ⊖ B_{3/2}) ⊕ B_{3/2} (D with its two corners
rounded to 3/2),

$$(D \times [0, 5]) \setminus \Bigl(\{\,(x, y) \in D,\ z \ge 4,\ d((x, y), C) \le 1,\ (1 - d((x, y), C))^2 + (z - 4)^2 \ge 1\,\} \cup \bigl((D \setminus D') \times [0, 5]\bigr)\Bigr)$$

The offset of a Bézier curve has no closed form, so the volume and area have none, and the case is
unsettled until two kernels agree.
