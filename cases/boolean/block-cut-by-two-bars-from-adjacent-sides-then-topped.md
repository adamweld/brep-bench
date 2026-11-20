+++
title = "A block cut from two sides by overlapping bars, then topped"
tags = ["boolean", "cut", "analytic", "planar", "successive"]
origin = "OCCT modalg_1/buc60801"

[volume]
value = 3732
exact = "20·20·12 − (576 + 588 − 96) = 4800 − 1068: bar A inside the block is 12·6·8, bar B below the top plane is 7·14·6, and they overlap in 4·6·4"
tex = '20\cdot 20\cdot 12 - (576 + 588 - 96) = 3732'

[area]
value = 2136
exact = "outer faces 400 + 302 + 192 + 240 + 198 + 240 = 1572 (bottom; top less B's 7·14 opening; x = 0 less A's 6·8 opening; x = 20; y = 0 less B's 7·6 opening; y = 20) plus the cutters' walls 304 + 260 = 564 (A: 80 + 80 + 72 + 48 + 24; B: 60 + 84 + 74 + 42, each less what lies inside the other)"
tex = '1572 + 304 + 260 = 2136'
+++

A block from the origin to (20, 20, 20). Cut it three times, one cut after another, each cutter a
box:

1. a bar A from (−5, 3, 2) to (12, 9, 10), which enters the block through its face x = 0 and
   ends inside it, at x = 12;
2. a bar B from (8, −5, 6) to (15, 14, 14), which enters through its face y = 0 and ends inside
   it, at y = 14. It overlaps A's cut in the box from (8, 3, 6) to (12, 9, 10), and it reaches
   above z = 12;
3. a large box from (−5, −5, 12) to (25, 25, 25), which takes away everything of the block at
   z ≥ 12. It cuts across B's cut where B rises above z = 12, and it leaves A's cut, which ends at
   z = 10, alone.

What is left is the block from z = 0 to 12 with the union of the two cuts taken out: A's tunnel in
from the side x = 0, and B's, whose top is now open, in from the side y = 0.

**As a set:**

$$[0,20]\times[0,20]\times[0,12] \setminus \bigl([-5,12]\times[3,9]\times[2,10] \,\cup\, [8,15]\times[-5,14]\times[6,14]\bigr)$$
