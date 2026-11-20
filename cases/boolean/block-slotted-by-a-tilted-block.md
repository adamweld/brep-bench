+++
title = "A block slotted by a tilted block"
tags = ["boolean", "cut", "planar", "analytic"]
origin = "OCCT boolean/gdml_public/A7"

[volume]
value = 416.0
exact = "8·(72 − 20): the part of the section in the xz plane common to the block's rectangle and the tilted one has area 20, exact in fractions"
tex = '8\left(12\cdot 6 - 20\right) = 416'

[area]
value = 450.6666666666667
exact = "the two faces in y = 0 and y = 8, 2·52 (the section is 72 less the cutter's 20), and the section's outline, 8 high all round: the rectangle's 36, less the 20/3 of it inside the cutter, plus the 14 of the cutter's outline inside the block, 130/3 in all"
tex = '2\cdot 52 + 8\cdot\tfrac{130}{3} = \tfrac{1352}{3}'
+++

A block, the box from the origin to (12, 8, 6), with a slanting slot cut into it by a second block set aslant.

The cutter is a block 10 long, 12 wide and 4 thick. It is centered at (6, 4, 6). Its length lies in the xz plane, rising toward +x and +z in the ratio 3 to 4: its long edges run along the direction (4, 0, 3)/5. Its thickness is along (−3, 0, 4)/5, and its width is along y, which carries it from y = −2 to y = 10, past both of the block's faces in y. Its four long corners in the xz plane are (8.8, 10.6), (11.2, 7.4), (0.8, 4.6) and (3.2, 1.4), so the cutter comes out of the block's top face, z = 6, and ends inside the block in a flat end, the rectangle from (0.8, y, 4.6) to (3.2, y, 1.4), and touches no other face. Its lower side makes a slanting floor.

The solid is the block less the cutter: the top face has a slanting slot across it, through the block's whole width in y.

**As a set**, with u = 0.8(x − 6) + 0.6(z − 6) and w = −0.6(x − 6) + 0.8(z − 6):

$$\bigl([0,12]\times[0,8]\times[0,6]\bigr) \setminus \{\,|u| \le 5,\ |w| \le 2\,\}$$
