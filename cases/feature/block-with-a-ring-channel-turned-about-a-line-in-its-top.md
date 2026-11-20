+++
title = "A block with a half-ring channel turned about a line in its top"
tags = ["feature", "channel", "revolve", "cylinder", "planar", "analytic"]
origin = "OCCT feat/featrevol/E9, OCCT feat/featrevol/G6, OCCT feat/featrevol/F6, OCCT feat/featrevol/F7"

[volume]
value = 2308.6725877128165
exact = "20·16·8 − (π/2)(5² − 3²)·10 = 2560 − 80π"
tex = '20\cdot 16\cdot 8 - \tfrac{\pi}{2}\left(5^2 - 3^2\right)\cdot 10 = 2560 - 80\pi'

[area]
value = 1477.5928947446202
exact = "1216 − 40 [two strips of the top, 10 × 2 each] + 50π [the outer half-cylinder] + 30π [the inner half-cylinder] + 16π [two half-annulus ends, 2·(π/2)(5² − 3²)]: = 1176 + 96π"
tex = '1216 - 40 + 50\pi + 30\pi + 16\pi = 1176 + 96\pi'
+++

A block with a channel in the shape of half a tube cut in its top, turned about a line lying in the
top face.

- The block: 0 ≤ x ≤ 20, −8 ≤ y ≤ 8, 0 ≤ z ≤ 8.
- The sketch: the rectangle 5 ≤ x ≤ 15, 3 ≤ y ≤ 5 lying in the block's top plane, z = 8.
- The axis: the line y = 0, z = 8, in that plane and parallel to the x axis, and so along the
  middle of the block's top.
- Turn the rectangle about the axis, through a full turn, and cut away what it sweeps. The sweep is
  a tube between radii 3 and 5 about the axis, from x = 5 to x = 15; the half of it above the top is
  in the air, so the block loses the lower half: a half-pipe channel.

The channel leaves a rod, the half-cylinder of radius 3 about the axis, standing in it along the
block's top, joined to the block at both ends of the channel, at x = 5 and x = 15.

**As a set:** with $r = \sqrt{y^2 + (z - 8)^2}$,

$$\bigl([0,20]\times[-8,8]\times[0,8]\bigr) \setminus \{\,5 \le x \le 15,\ 3 \le r \le 5\,\}$$
