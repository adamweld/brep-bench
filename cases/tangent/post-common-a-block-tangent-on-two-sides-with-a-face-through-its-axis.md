+++
title = "A post and a block tangent to it on two sides with a face through its axis, their common part"
tags = ["tangent", "boolean", "common", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/bopcommon_simple/ZC6"

[volume]
value = 140.43763859916055
exact = "V = 5A, A = 12 + 25·arcsin(3/5) is the area of the disc of radius 5 inside the strip 0 ≤ y ≤ 3"
tex = '5A,\quad A = 12 + 25\arcsin\tfrac35'

[area]
value = 178.35011087932844
exact = "area = 2A + 5P, P = 18 + 10·arcsin(3/5) is the length of the outline of that region"
tex = '2A + 5P,\quad P = 18 + 10\arcsin\tfrac35'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −5 to 5, y from 0 to 3 and z from 0 to 5: it is half as tall as the post, its bottom face level with the post's base, its top face at z = 5 inside the post. Take the part common to the post and the block.

The block's faces x = −5 and x = 5 are tangent to the post's wall along the vertical lines through (−5, 0) and (5, 0), each at an edge of its face. The face y = 0 passes through the post's axis, and the face y = 3 cuts across the post's circle, meeting the wall at (±4, 3). The block's corners (±5, 0) are on the wall and (±5, 3) are outside the post (34 > 25).

What is left is the part of the post with y between 0 and 3 and z between 0 and 5: a piece with a flat back (y = 0), a flat front (y = 3) and two wall patches that run up to the tangent lines.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cap \bigl([-5,5]\times[0,3]\times[0,5]\bigr)$$
