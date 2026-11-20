+++
title = "A ball in common with a right-angle cone at its center"
tags = ["boolean", "common", "sphere", "cone", "sector"]
origin = "OCCT tests/bugs/modalg_5/bug25232_6"

[volume]
value = 76.67926537588418
exact = "spherical sector (2π/3)R³(1 − cos α), R = 5, α = 45°"
tex = '\tfrac{2\pi}{3}\,5^3\left(1 - \cos 45^\circ\right) = \tfrac{250\pi}{3}\Bigl(1 - \tfrac{\sqrt2}{2}\Bigr)'

[area]
value = 101.54359595251008
exact = "the zone 2πR²(1 − cos α) = 25π(2 − √2) and the cone's side πR²sin α = 25π√2/2"
tex = '2\pi\cdot 5^2\left(1 - \tfrac{\sqrt2}{2}\right) + \pi\cdot 5^2\,\tfrac{\sqrt2}{2} = 50\pi - \tfrac{25\sqrt2}{2}\pi'
+++

A ball of radius 5 centered on the origin. A cone with its apex at the origin, its axis the +z axis
and a right angle at the apex: its half-angle is 45°, so its surface is ρ = z. Make it a solid cone
from the apex up to z = 6, where its base disc has radius 6, so that it reaches beyond the ball
everywhere. Make the part common to the ball and the cone.

The result is a ball sector: the cone's apex is the ball's center, the cone's side runs out from it
to the sphere, and the ball's cap, from the cone's rim up to z = 5, closes it.

**As a set:**

$$B_5 \cap \{\,\rho \le z \le 6\,\}$$
