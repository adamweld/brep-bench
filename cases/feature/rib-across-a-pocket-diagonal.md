+++
title = "A rib across a square pocket's diagonal, from corner to corner"
tags = ["feature", "rib", "pocket", "planar", "analytic"]
origin = "OCCT tests/feat/featlf/A4"

[volume]
value = 21866.696802066876
exact = "40·40·20 − 30·30·12 + V_rib = 21188 + 21596√2/45; V_rib = ∬ (h − 8) over the rib's footprint = 2∫∫ (h(s) − 8) du ds over |u| ≤ min(√2/2, s − 5, 35 − s), s = (x + y)/2, u = (x − y)/2, h − 8 = (70 + 4s)/15 for s ≤ 20 and symmetric: −12 + 21596√2/45"
tex = '40\cdot 40\cdot 20 - 30\cdot 30\cdot 12 + \iint (h - 8)\,dx\,dy = 21188 + \tfrac{21596\sqrt2}{45}'

[area]
value = 8461.431001326475
exact = "7840 (the block with its pocket) − F (the floor under the rib, F = 60√2 − 2) − W (the walls it covers in the two corners, 24√2 + 8/15) + F√233/15 (the rib's two sloping tops, sloping 4/15 along the diagonal) + 7196√2/15 − 24 (its two sides) = 117262/15 + 5936√2/15 + (60√2 − 2)√233/15"
tex = '7840 - F - W + \tfrac{\sqrt{233}}{15}F + \tfrac{7196\sqrt2}{15} - 24 = \tfrac{117262}{15} + \tfrac{5936\sqrt2}{15} + \tfrac{\sqrt{233}}{15}\left(60\sqrt2 - 2\right),\quad F = 60\sqrt2 - 2,\ W = 24\sqrt2 + \tfrac{8}{15}'
+++

A block from the origin to (40, 40, 20) with a square pocket cut into its top: the box from
(5, 5, 8) to (35, 35, 20) taken away, so the pocket's floor is at z = 8 and its walls are the
planes x = 5, x = 35, y = 5 and y = 35.

In the pocket stands a rib along its diagonal, the line from (5, 5) to (35, 35), made from an
open profile in the vertical plane x = y through that diagonal: the two segments from
(5, 5, 14) up to (20, 20, 18) and down to (35, 35, 14), a ridge whose ends are on the pocket's
corner edges. The rib is that profile thickened 1 to each side of its plane, |x − y| ≤ √2, and
carried down to the pocket's floor: at each point of the floor within 1 of the diagonal plane it
rises to the height of the profile above that point's foot on the diagonal, z = h(s), where
s = (x + y)/2 and

h = 14 + 4(1 − |s − 20|/15),

18 at the middle and 14 at the corners. At each end the rib runs into the pocket's corner, where
the walls x = 5 and y = 5 (and x = 35 and y = 35) cut it off; it fills the corner up to its two
sloping tops. The block and the rib are one solid.

**As a set:**

$$\Bigl(\bigl([0,40]^2\times[0,20]\bigr) \setminus \bigl([5,35]^2\times[8,20]\bigr)\Bigr) \cup \{\, 5 \le x \le 35,\ 5 \le y \le 35,\ |x - y| \le \sqrt2,\ 8 \le z \le 14 + 4\bigl(1 - \tfrac{|x + y - 40|}{30}\bigr) \,\}$$
