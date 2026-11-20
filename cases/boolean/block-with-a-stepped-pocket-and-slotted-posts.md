+++
title = "A block with a stepped pocket and two slotted posts"
tags = ["boolean", "cut", "union", "cylinder", "planar", "analytic"]
origin = "OCCT boolean/gdml_public/B7"

[volume]
value = 2756.5245415136287
exact = "4000 − 16·16·4 − 8·8·4 + 2(π1.5²·3 − s), s = 4(¼√2 + 1.125 arcsin(1/3)): the block less its two nested pockets, and two posts each less a slot of width 1 and depth 1 (the area of the strip |x′| ≤ ½ in the post's circle)"
tex = '4000 - 1024 - 256 + 2\left(\pi\cdot 1.5^2\cdot 3 - s\right),\quad s = 4\left(\tfrac{\sqrt2}{4} + \tfrac98\arcsin\tfrac13\right)'

[area]
value = 2047.7843333501517
exact = "the block's bottom and sides 400 + 800, its rim 144, the wide pocket's walls 256 and floor 192 − 4.5π (less the posts' feet), the deep pocket's walls 128 and floor 64; and each post's side 9π − 6 arcsin(1/3), top and slot floor together π2.25 (the slot floor equal to the top's loss), its two slot walls 4√2: 2(11.25π − 6 arcsin(1/3) + 4√2)"
tex = '400 + 800 + 144 + 256 + \left(192 - 4.5\pi\right) + 128 + 64 + 2\left(11.25\pi - 6\arcsin\tfrac13 + 4\sqrt2\right)'
+++

A block hollowed by two nested pockets, with two slotted posts fused to the floor of the wide one.

The block is the box from the origin to (20, 20, 10). The wide pocket is the box from (2, 2, 6) to (18, 18, 10) taken out of its top; in it, the deep pocket, the box from (6, 6, 2) to (14, 14, 6), is taken out below the wide pocket's floor, so the floor of the wide pocket is a square ring and the deep pocket has its floor at z = 2. Each pocket runs out through the block's top.

Two posts stand on the wide pocket's floor (z = 6), each a cylinder of radius 1.5 up to z = 9, at (x, y) = (4, 10) and (16, 10). Each is cut by a slot 1 wide, from z = 8 to the top, running across its whole diameter along y: the strip |x − 4| ≤ 0.5 for the first post and |x − 16| ≤ 0.5 for the second. The posts are clear of the deep pocket and of the walls.
