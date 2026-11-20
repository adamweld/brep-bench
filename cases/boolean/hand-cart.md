+++
title = "A hand cart"
tags = ["boolean", "union", "box", "cylinder", "cone", "analytic"]
origin = "OCCT boolean/gdml_public/A1, boolean/gdml_public/A2"

[volume]
value = 274.1194640914112
exact = "216 + 4·π2²·1 + π·0.5²·6 + (1/3)π·1²·3 = 216 + 18.5π: chassis, four wheels, handle, nose"
tex = '216 + 4\cdot\pi\cdot 2^2\cdot 1 + \pi\cdot 0.5^2\cdot 6 + \tfrac13\pi\cdot 1^2\cdot 3 = 216 + 18.5\pi'

[area]
value = 362.3417815916571
exact = "chassis 252, each wheel 12π less twice the part of its flat face against the chassis (the disk above z = 1.5: S = 4π − (4 arccos¼ − ½√3.75)), the handle's side and end 6π + ¼π less its ¼π on the chassis, the nose's side π√10 less its base π on the chassis: 252 + 4(12π − 2S) + 6π + π√10 − π"
tex = '252 + 4\left(12\pi - 2S\right) + 6\pi + \sqrt{10}\,\pi - \pi,\quad S = 4\pi - \left(4\arccos\tfrac14 - \tfrac12\sqrt{3.75}\right)'
+++

A small cart, one solid, made by fusing its parts to a chassis one after another.

The chassis is the box from (0, 0, 1.5) to (12, 6, 4.5). Four wheels are disks 1 thick with their flat faces against the chassis's long sides: each is a cylinder of radius 2 with its axis along y through (3, y, 2) or (9, y, 2), occupying y from −1 to 0 on the side y = 0 and y from 6 to 7 on the side y = 6. The part of each wheel's face above z = 1.5 lies against the chassis, and the rest hangs below it; the wheels' tops, at z = 4, stay below the chassis's top. A handle is a cylinder of radius 0.5 along +x from the chassis's end x = 12 to x = 18, its axis through (y, z) = (3, 3). A nose is a cone of base radius 1 on the end x = 0, centered at (y, z) = (3, 3), pointing along −x to its apex at (−3, 3, 3).

**As a set:**

$$\{\,0 \le x \le 12,\ 0 \le y \le 6,\ 1.5 \le z \le 4.5\,\} \cup \bigcup_{c,\,I} \{\,(x - c)^2 + (z - 2)^2 \le 2^2,\ y \in I\,\} \cup \{\,12 \le x \le 18,\ (y - 3)^2 + (z - 3)^2 \le 0.5^2\,\} \cup \{\,-3 \le x \le 0,\ (y - 3)^2 + (z - 3)^2 \le \bigl(\tfrac{x + 3}{3}\bigr)^2\,\}$$

with c = 3 or 9 and I = [−1, 0] or [6, 7].
