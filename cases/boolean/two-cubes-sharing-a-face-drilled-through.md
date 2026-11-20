+++
title = "Two cubes sharing a face, drilled through the face's centre"
tags = ["boolean", "cut", "coincident", "multi-solid", "analytic"]
origin = "OCCT tests/bugs/modalg_7/bug24632_3"

[volume]
value = 1434.5133223538372
exact = "2(1000 − π·3²·10) = 2000 − 180π"
tex = '2\left(1000 - \pi\cdot 3^2\cdot 10\right) = 2000 - 180\pi'

[area]
value = 1463.8937829015426
exact = "2(600 − 2π·3² + 2π·3·10) = 1200 + 84π: each cube's own six faces, less its two discs and with the bore's wall"
tex = '2\left(600 - 2\pi\cdot 3^2 + 2\pi\cdot 3\cdot 10\right) = 1200 + 84\pi'
+++

Two cubes of side 10 resting one on the other: one from (−5, −5, 0) to (5, 5, 10), the other from
(−5, −5, 10) to (5, 5, 20). They share the whole of the face z = 10. Do not fuse them into one
solid: the result is two solids that touch, each with its own faces, so the shared face is counted
once in each.

Drill a hole of radius 3 right through both, along the z axis, which passes through the middle of
the shared face: a cylinder that runs past the bottom of the lower cube and the top of the upper
one. The result is two cubes each with a round bore, touching along a ring.

The volume and the area are those of the two solids taken together.
