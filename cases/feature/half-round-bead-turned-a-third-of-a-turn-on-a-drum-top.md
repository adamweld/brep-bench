+++
title = "A half-round bead turned a third of a turn on a drum's top"
tags = ["feature", "revolve", "union", "torus", "cylinder", "analytic"]
origin = "OCCT tests/feat/featrf/B2"

[volume]
value = 13987.59364811604
exact = "π·20²·10 + (2π/3)·12·18π (Pappus: the half disc's area 18π, its centroid 12 from the axis, turned through 2π/3) = 4000π + 144π²"
tex = '4000\pi + 144\pi^2'

[area]
value = 4055.1566363446536
exact = "400π (the drum's bottom) + 400π (its side) + (400π − (π/3)(18² − 6²)) (its top less the bead's foot) + (2π/3)·6·12π (the bead's round face, 12π = ∫₀^π (12 + 6 cos θ) dθ) + 2·18π (its two ends) = 1140π + 48π²"
tex = '400\pi + 400\pi + \Bigl(400\pi - \tfrac{\pi}{3}(18^2 - 6^2)\Bigr) + \tfrac{2\pi}{3}\cdot 6\cdot 12\pi + 2\cdot 18\pi = 1140\pi + 48\pi^2'
+++

A drum, the cylinder of radius 20 about the z axis from z = 0 to 10.

A bead stands on its top: the half disc of radius 6 about the point (x, z) = (12, 10) in the plane
y = 0, the part above z = 10 (its diameter lies on the drum's top, from x = 6 to 18), turned about
the z axis from +x towards +y through 120°. So the bead is a third of a half torus: its round
surface is the torus about the circle ρ = 12, z = 10, and its two ends are half discs in the
half-planes at 0° and 120°. It is joined to the drum along the ring sector of the drum's top from
ρ = 6 to 18, and the drum and the bead are one solid.

**As a set:**

$$\{\rho \le 20,\ 0 \le z \le 10\} \cup \{\,0^\circ \le \varphi \le 120^\circ,\ (\rho - 12)^2 + (z - 10)^2 \le 36,\ z \ge 10\,\}$$
