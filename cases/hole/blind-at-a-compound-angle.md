+++
title = "A blind hole drilled at a compound angle"
tags = ["hole", "blind", "inclined", "cut", "planar", "cylinder", "ellipse", "analytic"]
origin = "OCCT tests/boolean/gdml_public/B3"

[volume]
value = 59095.22131576614
exact = "50·40·30 − π·4²·18 = 60000 − 288π: the hole is a cylinder between its square floor and the top face's slanted plane, which crosses the axis 18 from the floor, so it holds as much as a straight cylinder 18 long"
tex = '50\cdot 40\cdot 30 - \pi\cdot 4^2\cdot 18 = 60000 - 288\pi'

[area]
value = 9827.2566008882119
exact = "9400 − π·4²/(2/3) + 2π·4·18 + π·4² = 9400 + 136π: the block, less the elliptic mouth (the axis meets the top at cos θ = 2/3), and the wall, its mean length 18, and the floor"
tex = '9400 - \frac{\pi\cdot 4^2}{2/3} + 2\pi\cdot 4\cdot 18 + \pi\cdot 4^2 = 9400 + 136\pi'
+++

A block, the box from the origin to (50, 40, 30), with a round blind hole of radius 4 drilled into
its top face aslant, its axis leaning toward neither of the block's sides.

The hole's axis enters the top face (z = 30) at (25, 20, 30) and runs into the block along the
direction u = (−2, 1, −2)/3, so it makes an angle with the vertical whose cosine is 2/3 (about
48.19°). The hole is drilled 18 along that axis: its floor is the flat disk of radius 4 square to
the axis, centred at (25, 20, 30) + 18u = (13, 26, 18). Its mouth in the top face is an ellipse,
semi-axes 6 along (−2, 1)/√5 and 4 across it. The hole stays inside the block and reaches no other
face.

If you cut the hole with a cylinder, make it run out past the top face.

**As a set:** with c = (25, 20, 30) and s = (p − c)·u the distance along the axis,

$$\bigl([0,50]\times[0,40]\times[0,30]\bigr) \setminus \{\, \lVert p - c - s\,u \rVert \le 4,\ s \le 18 \,\}$$
