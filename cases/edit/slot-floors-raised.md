+++
title = "A comb's two slot floors raised, one until its slot is gone"
tags = ["edit", "move-face", "slot", "planar", "analytic"]
origin = "OCCT tests/offset/shape_type_i_c_multi/C5"

[volume]
value = 11370.0
exact = "(40·20 − 6·7)·15: the block with one slot 6 wide and 7 deep, 15 long"
tex = '\left(40\cdot 20 - 6\cdot 7\right)\cdot 15'

[area]
value = 3526.0
exact = "2(40·20 − 6·7) + (2·40 + 2·20 + 2·7)·15"
tex = '2\left(40\cdot 20 - 6\cdot 7\right) + \left(2\cdot 40 + 2\cdot 20 + 2\cdot 7\right)\cdot 15'
+++

A comb: the block from the origin to (40, 15, 20) with two slots cut down into its top, right
across it along y: the boxes from (8, 0, 8) to (14, 15, 20) and from (26, 0, 8) to (32, 15, 20)
taken away. Each slot is 6 wide and 12 deep, its floor at z = 8.

Then edit it directly, moving only the two slots' floors, each along its outward normal, +z, and
leaving every other face where it is: the first slot's floor (8 ≤ x ≤ 14) by 5, up to z = 13,
and the second's (26 ≤ x ≤ 32) by 12, up to z = 20. The slots' walls are trimmed to meet the
moved floors. The second floor arrives in the plane of the top, so its slot's walls shrink to
nothing and the slot is gone: the top runs on across it, one face from x = 14 to x = 40.

The result is the block from the origin to (40, 15, 20) with one slot, the box from (8, 0, 13) to
(14, 15, 20) taken away.

**As a set:**

$$\bigl([0,40]\times[0,15]\times[0,20]\bigr) \setminus \bigl([8,14]\times[0,15]\times[13,20]\bigr)$$
