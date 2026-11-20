+++
title = "A prism with a rounded corner, its far side drafted, under a half ball"
tags = ["draft", "planar", "cylinder", "sphere", "boolean", "analytic"]
origin = "OCCT tests/bugs/modalg_2/bug422_1"

[volume]
value = 1352.884643666072
exact = "120(10 + 100c + 1000c²/3) − 40(1 − π/4) + 18π, c = tan 5°·√544/240: the section at height z is the right triangle with legs 20s and 12s, s = 1 + (10 − z)c, less the rounded corner's 4(1 − π/4); the half ball adds 2π·27/3"
tex = '120\left(10 + 100c + \tfrac{1000}{3}c^2\right) - 40\left(1 - \tfrac\pi4\right) + 18\pi,\quad c = \tfrac{\sqrt{544}}{240}\tan 5^\circ'

[area]
value = 856.9326728239277
exact = "top 120 − 4(1 − π/4) + 9π (the half ball's dome replaces its disk of 9π), bottom 120s₀² − 4(1 − π/4), s₀ = 1 + 10c, the leg x = 0: 12(10 + 50c) − 20, the leg y = 0: 20(10 + 50c) − 20, the round 10π, the drafted face √544(10 + 50c)/cos 5°"
tex = '\left(120 - 4 + \pi + 9\pi\right) + \left(120s_0^2 - 4 + \pi\right) + 32(10 + 50c) - 40 + 10\pi + \tfrac{\sqrt{544}}{\cos 5^\circ}(10 + 50c)'
+++

A tall prism on a right triangle: the triangle with corners (0, 0), (20, 0) and (0, 12) in the plane
z = 0, extruded to z = 10, with the vertical edge at the right angle (the corner at the origin)
rounded to a quarter cylinder of radius 2, whose axis is x = y = 2, tangent to the two legs at (2, 0)
and (0, 2). Its sides are the legs (the planes y = 0 and x = 0), the round, and the hypotenuse, the
plane x/20 + y/12 = 1.

Draft the hypotenuse face by 5°, with the pull direction +z and the neutral plane z = 10, the top:
the face turns about its top edge until it leans 5° from the z direction, inward as it rises, so
that lower down it lies farther out. At height z it is the plane parallel to the hypotenuse's
direction, 10 − z times tan 5° outward of the original hypotenuse, measured square to it. No other
face is drafted: the legs and the round run on unchanged, and the bottom, z = 0, is the larger
triangle that results, with its corner rounded the same.

Fuse a half ball of radius 3 on the top: the upper half, z ≥ 10, of the ball centred at (6, 4.5, 10).
Its flat disk lies wholly within the top face, more than 3.3 from the hypotenuse's edge and clear of
the legs and the round.

**As a set:**

$$\Bigl(\{\,x \ge 0,\ y \ge 0,\ \tfrac{x}{20} + \tfrac{y}{12} \le 1 + \tfrac{(10 - z)\tan 5^\circ\sqrt{544}}{240},\ 0 \le z \le 10\,\} \setminus \{\,x < 2,\ y < 2,\ (x-2)^2 + (y-2)^2 > 4\,\}\Bigr) \cup \{\,(x-6)^2 + (y - 4.5)^2 + (z - 10)^2 \le 9,\ z \ge 10\,\}$$
