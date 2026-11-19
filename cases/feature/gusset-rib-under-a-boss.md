+++
title = "A gusset rib from a plate up to a square boss's side"
tags = ["feature", "rib", "union", "planar", "analytic"]
origin = "OCCT tests/feat/featlf/B5"

[volume]
value = 32520.0
exact = "60·40·8 + 20·20·30 + ½·30·22·4 = 32520: the plate, the boss, and the rib, a right triangle with legs 30 and 22 run 4 thick"
tex = '60\cdot 40\cdot 8 + 20\cdot 20\cdot 30 + \tfrac12\cdot 30\cdot 22\cdot 4 = 32520'

[area]
value = 9400.8086019019062
exact = "6400 + 2400 − 30·4 − 22·4 + 2·330 + 4√(30² + 22²) = 9252 + 8√346: the plate, the boss's four sides, less the rib's foot on the plate and its joint on the boss, plus its two triangular faces and its sloping top"
tex = '6400 + 2400 - 30\cdot 4 - 22\cdot 4 + 2\cdot 330 + 4\sqrt{30^2 + 22^2} = 9252 + 8\sqrt{346}'
+++

A plate with a square boss standing on it, braced by a gusset: a rib made from one straight line
running from the plate's top up to the boss's side.

- **The plate:** the box from (0, −20, 0) to (60, 20, 8).
- **The boss:** the box from (0, −10, 8) to (20, 10, 38), standing on the plate's top at its end;
  its face x = 0 lies in the plane of the plate's end face x = 0, the two making one face.
- **The rib:** the open profile is the segment in the plane y = 0 from (50, 0, 8) on the plate's
  top to (20, 0, 30) on the boss's side x = 20. The rib fills the space between that segment, the
  plate's top and the boss's side, the right triangle (20, 8), (50, 8), (20, 30) in (x, z), and is
  4 thick, 2 to each side of the plane y = 0.

The plate, the boss and the rib are one solid.

**As a set:**

$$\bigl([0,60]\times[-20,20]\times[0,8]\bigr) \cup \bigl([0,20]\times[-10,10]\times[8,38]\bigr) \cup \{\, |y| \le 2,\ x \ge 20,\ z \ge 8,\ 22(x - 20) + 30(z - 8) \le 660 \,\}$$
