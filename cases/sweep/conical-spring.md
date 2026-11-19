+++
title = "A conical spring: round wire along a tapering helix"
tags = ["sweep", "helix", "conical", "spring", "analytic"]
origin = "OCCT tests/helix/standard/B8"

[volume]
value = 2852.1973804550365
exact = "πr²L, r = 3/2, L = ∫₀^{8π} √(R² + a²) dt = π[F(20) − F(12)], R = 20 − t/π, a² = 1/π² + 16/π² = 17/π², F(R) = (R/2)√(R² + a²) + (a²/2) asinh(R/a): a tube's volume is its section's area times its length, wherever its radius stays below the path's least radius of curvature"
tex = '\pi r^2 L,\quad L = \pi\bigl[F(20) - F(12)\bigr],\quad F(R) = \tfrac{R}{2}\sqrt{R^2 + a^2} + \tfrac{a^2}{2}\operatorname{arsinh}\tfrac{R}{a},\quad a^2 = \tfrac{17}{\pi^2},\ r = \tfrac32'

[area]
value = 3817.0670075478694
exact = "2πrL + 2πr², L as for the volume: the tube's side, its circumference times its length, and its two flat ends"
tex = '2\pi r L + 2\pi r^2'
+++

A conical compression spring: a round wire of radius 1.5 coiled along a helix that tapers as it
rises.

The wire's centre line is the helix

c(t) = ((20 − t/π) cos t, (20 − t/π) sin t, 4t/π),  0 ≤ t ≤ 8π:

it starts at (20, 0, 0), turns right-handed about the z axis (from +x toward +y) through four whole
turns, rising 8 each turn and closing in by 2 each turn, so it lies on a cone and ends at
(12, 0, 32). The wire's section is the disk of radius 1.5 centred on the helix and square to it
there, carried along the helix with it, and its two ends are flat, square to the helix at its two
ends.

Neighbouring coils are about 8.2 apart, centre to centre, so the wire never touches itself, and the
helix bends no tighter than a radius of about 12, far more than the wire's radius.

**As a set:**

$$\bigl\{\, c(t) + u : 0 \le t \le 8\pi,\ u \perp c'(t),\ \lVert u \rVert \le 1.5 \,\bigr\}$$
