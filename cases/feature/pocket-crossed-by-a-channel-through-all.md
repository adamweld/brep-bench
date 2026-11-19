+++
title = "A blind pocket crossed by a channel cut through all from the side"
tags = ["feature", "pocket", "through-all", "cut", "planar", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestPocket.py::testPocketThroughAllCase"

[volume]
value = 38820.0
exact = "80·50·12 − 30·30·7 − 12·9·50 + 12·7·30 = 38820: the plate, less the pocket and the channel, plus what they share (the channel's part above the pocket's floor, inside the pocket)"
tex = '80\cdot 50\cdot 12 - 30\cdot 30\cdot 7 - 12\cdot 9\cdot 50 + 12\cdot 7\cdot 30 = 38820'

[area]
value = 12056.0
exact = "4000 + 1200 + 2(960 − 108) + (4000 − 900 − 240) + (900 − 360) + (2·210 + 2(210 − 84)) + 600 + 2(450 − 210) = 12056: bottom; the ends x = 0, 80; the sides y = 0, 50 less the channel's mouths; the top less the pocket and the channel beside it; the pocket's floor less the channel; the pocket's walls, two whole and two less the channel's opening; the channel's floor; its two walls less where the pocket opens them"
tex = '4000 + 1200 + 2(960 - 108) + 2860 + 540 + 672 + 600 + 480 = 12056'
+++

A plate with a square blind pocket in its top, and a channel cut across the plate from one side
right through to the other, crossing the pocket and running deeper than its floor.

- **The plate:** the box from the origin to (80, 50, 12).
- **The pocket:** the square 25 ≤ x ≤ 55, 10 ≤ y ≤ 40 on the top face, cut 7 deep, so its floor
  is at z = 5.
- **The channel:** the rectangle 34 ≤ x ≤ 46, 3 ≤ z ≤ 12 on the side face y = 0, square to the
  pocket's sketch, cut through all along +y, out through the side face y = 50. It is 12 wide, open
  at the top, with its floor at z = 3.

The channel crosses the pocket's middle: inside the pocket it is a trench 2 deeper than the
pocket's floor, which it splits in two, and it cuts an opening 12 wide and 7 tall in the pocket's
walls y = 10 and y = 40.

**As a set:**

$$\bigl([0,80]\times[0,50]\times[0,12]\bigr) \setminus \bigl([25,55]\times[10,40]\times[5,12]\bigr) \setminus \bigl([34,46]\times[0,50]\times[3,12]\bigr)$$
