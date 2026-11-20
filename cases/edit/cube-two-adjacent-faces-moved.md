+++
title = "A cube's two adjacent faces moved, one out and one in"
tags = ["edit", "move-face", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/G6, G7, G8, G9, H1"

[volume]
value = 1040.0
exact = "13·8·10"
tex = '13 \cdot 8 \cdot 10'

[area]
value = 628.0
exact = "2(13·8 + 13·10 + 8·10)"
tex = '2\left(13\cdot 8 + 13\cdot 10 + 8\cdot 10\right)'
+++

The cube from the origin to (10, 10, 10).

Then edit it directly: move its face at x = 10 out by 3 along +x, to the plane x = 13, and its face at y = 10, which is adjacent to it, in by 2 along −y, to the plane y = 8. The four faces beside each follow: they are extended or trimmed to meet the moved ones.

The result is the box from the origin to (13, 8, 10).

**As a set:**

$$[0,13] \times [0,8] \times [0,10]$$
