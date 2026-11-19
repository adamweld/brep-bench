+++
title = "A ball knob bored from below to its center"
tags = ["hole", "blind", "cut", "sphere", "cylinder", "analytic"]
origin = "OCCT tests/bugs/mesh/bug29751"

[volume]
value = 13396.752090236376
exact = "(4/3)π·15³ − π·4²·w − πh²(3·15 − h)/3, w = √(15² − 4²) = √209, h = 15 − w: the ball, less the hole's cylinder from its mouth up to the center and the cap of ball below the mouth"
tex = '\tfrac43\pi\cdot 15^3 - \pi\cdot 4^2\, w - \tfrac{\pi h^2(3\cdot 15 - h)}{3},\quad w = \sqrt{15^2 - 4^2} = \sqrt{209},\ h = 15 - w'

[area]
value = 3189.8463455708457
exact = "4π·15² − 2π·15h + 2π·4w + π·4², w = √209, h = 15 − w: the sphere less the cap the mouth takes, and the hole's wall and its flat end"
tex = '4\pi\cdot 15^2 - 2\pi\cdot 15\,h + 2\pi\cdot 4\,w + \pi\cdot 4^2,\quad w = \sqrt{209},\ h = 15 - w'
+++

A ball knob, bored for the lever it will sit on: a ball of radius 15 centred on the origin, with a
round blind hole of radius 4 bored up into it along the z axis from below.

The hole enters the ball through its lower side, around its south pole (0, 0, −15): its mouth is
the circle of radius 4 where the ball meets the cylinder ρ = 4, at z = −√209 ≈ −14.457. It runs up
the z axis and ends flat at the ball's centre, in the disk of radius 4 at z = 0. The ball's upper
half is whole.

If you cut the hole with a cylinder, make it run out past the ball below. Place the ball's poles on
the z axis: the south pole is in the hole and the north pole is on the ball's surface.

**As a set:**

$$B_{15} \setminus \{\, \rho \le 4,\ z \le 0 \,\}$$
