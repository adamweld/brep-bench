+++
title = "A square turned all the way round a boss on a box, a collar"
tags = ["feature", "revolve", "union", "coincident", "cylinder", "analytic"]
origin = "OCCT tests/feat/featrevol/I7"

[volume]
value = 18692.3449041264528
exact = "abc + πr²h + π(R² − r²)w, a = b = 40, c = 10, r = 6, h = 12, R = 11, w = 5: = 16000 + 857π"
tex = 'abc + \pi r^2 h + \pi\left(R^2 - r^2\right)w,\quad a = b = 40,\ c = 10,\ r = 6,\ h = 12,\ R = 11,\ w = 5:\ = 16000 + 857\pi'

[area]
value = 5409.4689747964199
exact = "(2ab + 2(a + b)c − πR²) + πr² + 2πr(h − w) + π(R² − r²) + 2πRw: = 4800 + 194π"
tex = '\left(2ab + 2(a + b)c - \pi R^2\right) + \pi r^2 + 2\pi r(h - w) + \pi\left(R^2 - r^2\right) + 2\pi R w:\ = 4800 + 194\pi'
+++

A box from (−20, −20, 0) to (20, 20, 10) with a round boss on its top: a cylinder of radius 6 about
the z axis, from z = 10 to z = 22, united with the box. Round the boss a collar is turned, by
revolving a square sketch right round the z axis, a full turn, and joining the result to the box and
the boss: the square is 5 by 5 in the plane y = 0, 6 ≤ x ≤ 11 and 10 ≤ z ≤ 15, so it lies on the
box's top and against the boss's wall.

The collar is a ring, everything with 6 ≤ ρ ≤ 11 and 10 ≤ z ≤ 15, standing on the box and
touching the boss along its whole wall from z = 10 to z = 15, so that its inner face lies in the
boss's wall. The box, the boss and the collar are one solid.

**As a set:**

$$\bigl([-20,20]^2 \times [0,10]\bigr) \cup \{\,\rho \le 6,\ 10 \le z \le 22\,\} \cup \{\,6 \le \rho \le 11,\ 10 \le z \le 15\,\}$$
