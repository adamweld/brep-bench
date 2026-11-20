+++
title = "A wall bracket with a boss"
tags = ["boolean", "union", "cut", "box", "cylinder", "analytic"]
origin = "OCCT boolean/gdml_public/C7"

[volume]
value = 1095.707963267949
exact = "480 + 480 + 96 + 24 − 4π + 18π − 9π = 1080 + 5π: base 20 × 12 × 2, upright 20 × 2 × 12, two ribs 2 × 6 × 4, lug 6 × 2 × 2, less two bolt holes π·1²·2, plus the boss π·3²·2, less the hole π·1.5²·4"
tex = '480 + 480 + 96 + 24 - 4\pi + 18\pi - 9\pi = 1080 + 5\pi'

[area]
value = 1337.8274273593602
exact = "the five boxes' faces 608 + 608 + 2·88 + 56 less twice the contacts 2(40 + 2·20 + 12) = 184, 1264; the boss adds 12π (its side), the two bolt holes 2·(4π − 2π) and the hole 12π − 2·2.25π"
tex = '1264 + 12\pi + 4\pi + 7.5\pi = 1264 + 23.5\pi'
+++

A wall bracket, one solid, in boxes and cylinders, standing on the plane z = 0.

A base plate, the box from the origin to (20, 12, 2). An upright, the box from (0, 0, 2) to (20, 2, 14), standing on the base plate's back edge, flush with its back and ends. Two ribs, the boxes from (0, 2, 2) to (2, 8, 6) and from (18, 2, 2) to (20, 8, 6), each against the upright's front face and standing on the base plate, flush with its ends. A lug, the box from (7, 0, 14) to (13, 2, 16), on top of the upright, flush with its back and front.

A boss, a cylinder of radius 3 along y from y = 2 to y = 4, centered at (x, z) = (10, 10), on the upright's front face. Two bolt holes of radius 1 through the base plate, along z through (5, 8) and (15, 8). A hole of radius 1.5 along y through the boss and the upright, centered at (x, z) = (10, 10), from the back, y = 0, to the boss's front face, y = 4.

If you cut the holes with cylinders, make them run past the faces they open in.
