+++
title = "A parabolic-segment prism with its curved top edge and its two upright edges rounded, the rounds creased at the corners"
tags = ["fillet", "corner", "parabolic-cylinder", "planar", "crease", "unsettled"]
origin = "OCCT tests/blend/simple/F8, OCCT tests/blend/simple/G1, OCCT tests/blend/simple/F9, OCCT tests/blend/simple/G2"
settled = false
+++

The parabolic segment between the parabola y = 9 − x²/4 and its chord, the segment of the x axis
from (−6, 0) to (6, 0): the region D, 0 ≤ y ≤ 9 − x²/4, extruded from z = 0 to z = 4. It has a flat
bottom and top, a flat upright side in the plane y = 0, and a curved wall, the parabolic cylinder,
which meets the flat side in two upright edges at (±6, 0), each at a slant: the parabola's slope there
is ∓3, so the wall and the side make an angle of 71.57° inside the solid. The parabola curves
tightest at its vertex (0, 9), with radius of curvature 2.

Round three edges: the one edge where the curved wall meets the top, the arc of the parabola at z = 4,
to radius 1, and the two upright edges, to radius 3/2.

- The top round is the blend a ball of radius 1 sweeps rolling along the arc, touching the top and the
  curved wall from inside. Its ends are cut by the plane y = 0 obliquely.
- Each upright round is a vertical surface, a circular arc in plan of radius 3/2 tangent to the flat
  side and to the wall (it turns through 108.43°), running from the bottom to the top.

The top edge of the flat side, along y = 0 at z = 4, is not rounded. So at each top corner, (±6, 0, 4),
the top round and the upright round do not close the corner with a patch of their own: each runs the
whole length of its edge, neither is shortened by the other, and where they cross they meet in a
crease, the curve where the two surfaces intersect. The solid is the prism less what each round
removes.

**As a set:** with W the parabola arc, d the distance in the plane, and D′ = (D ⊖ B_{3/2}) ⊕ B_{3/2}
(D with its two corners rounded to 3/2, the wall's curvature radius being at least 2),

$$(D \times [0, 4]) \setminus \Bigl(\{\,(x, y) \in D,\ z \ge 3,\ d((x, y), W) \le 1,\ (1 - d((x, y), W))^2 + (z - 3)^2 \ge 1\,\} \cup \bigl((D \setminus D') \times [0, 4]\bigr)\Bigr)$$

The offset of a parabola has no closed form, so the volume and area have none, and the case is
unsettled until two kernels agree.
