+++
title = "A shaft's end centre-drilled: a 60° countersink, a pilot and a 118° point"
tags = ["hole", "countersink", "drill-point", "blind", "revolve", "cut", "planar", "cylinder", "cone", "analytic"]

[volume]
value = 15612.454929925728
exact = "π·10²·50 − π·h₁/3·(R² + Rr + r²) − π·r²·4 − (1/3)π·r²·h₂, R = 3.35, r = 1.6, h₁ = (R − r)/tan 30° = 1.75√3, h₂ = r cot 59°"
tex = '\pi\cdot 10^2\cdot 50 - \tfrac{\pi h_1}{3}\left(R^2 + Rr + r^2\right) - \pi r^2\cdot 4 - \tfrac{1}{3}\pi r^2 h_2,\quad R = 3.35,\ r = 1.6,\ h_1 = \tfrac{R - r}{\tan 30^\circ} = 1.75\sqrt{3},\ h_2 = r\cot 59^\circ'

[area]
value = 3838.6777619321509
exact = "2π·10² + 2π·10·50 − πR² + π(R + r)·(R − r)/sin 30° + 2πr·4 + πr·r csc 59°, with R and r as for the volume: the shaft, less the mouth, and the countersink's cone, the pilot's wall and the point's cone"
tex = '2\pi\cdot 10^2 + 2\pi\cdot 10\cdot 50 - \pi R^2 + \pi\,(R + r)\,\frac{R - r}{\sin 30^\circ} + 2\pi r\cdot 4 + \pi r^2\csc 59^\circ'
+++

A shaft, the cylinder of radius 10 on the z axis from z = 0 to z = 50, with a centre hole drilled
into its top end (z = 50) on the z axis, the kind a lathe centre runs in. Seen from the top, going
down, the hole is:

- a countersink, a cone with an included angle of 60° (its side at 30° to the axis), opening into
  the end face in a circle of radius 3.35 and narrowing to radius 1.6 at z = 50 − 1.75√3 ≈ 46.969;
- a pilot of radius 1.6, at its full radius for 4 more, down to z = 46 − 1.75√3 ≈ 42.969;
- the point the drill leaves, a cone with an included angle of 118° (its side at 59° to the axis),
  its base the pilot's circle there and its apex on the axis 1.6 cot 59° ≈ 0.961 below.

**As a set:** with $z_p = 46 - 1.75\sqrt{3}$, where the pilot's full radius ends,

$$\{\,\rho \le 10,\ 0 \le z \le 50\,\} \setminus \bigl(\{\,\rho \le 3.35 - (50 - z)\tan 30^\circ\,\} \cup \{\,\rho \le 1.6,\ \rho \le 1.6 + (z - z_p)\tan 59^\circ\,\}\bigr)$$
