+++
title = "A zigzag rib running out to an L-plate's sides"
tags = ["feature", "rib", "union", "planar", "coplanar", "analytic"]
origin = "OCCT feat/featlf/D7"

[volume]
value = 6600
exact = "plate 1200·5 + rib 100·6, the rib's footprint 2 × (10 + 24 + 6 + 10) = 100"
tex = '1200\cdot 5 + 100\cdot 6'

[area]
value = 3824
exact = "L-plate 2·1200 + 160·5 = 3200, plus the rib's walls 104·6 = 624 (its top replaces the 100 of plate top it covers)"
tex = '\left(2\cdot 1200 + 160\cdot 5\right) + 104\cdot 6'
+++

An L-shaped plate with a rib standing on it, joined into one solid. The rib is a wall along an
open zigzag of four straight segments, and it is carried out at both ends until it meets the
plate's sides.

- The plate: the L in the xy plane made of the rectangles 0 ≤ x ≤ 40, 0 ≤ y ≤ 20 and
  0 ≤ x ≤ 20, 0 ≤ y ≤ 40, from z = 0 to z = 5. Its inside corner is at (20, 20).
- The rib's centre line: from (0, 6) to (10, 6), up to (10, 30), across to (16, 30) and up to
  (16, 40). Its first segment starts on the plate's side x = 0 and its last ends on the side
  y = 40, so the rib runs right out to the plate's edge at both ends and no further.
- The rib is 2 thick (1 each side of the centre line, its corners square, not rounded) and runs from
  z = 5, the plate's top, up to z = 11.

The rib's two end faces lie in the planes of the plate's sides x = 0 and y = 40, flush with them.

**As a set:**

$$\bigl(([0,40]\times[0,20] \cup [0,20]\times[0,40])\times[0,5]\bigr) \cup \bigl(R\times[5,11]\bigr)$$

with R the union of the four rectangles $[0,11]\times[5,7]$, $[9,11]\times[5,31]$,
$[9,17]\times[29,31]$ and $[15,17]\times[29,40]$.
