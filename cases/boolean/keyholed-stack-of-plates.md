+++
title = "A stack of plates with a slotted disk"
tags = ["boolean", "union", "cut", "cylinder", "planar", "analytic"]
origin = "OCCT boolean/gdml_public/B4"

[volume]
value = 617.3748332590253
exact = "400 + 2(36π − s) + 2.25π, s = √35 + 36 arcsin(1/6) − 4: the slot's area in the disk, the part of the strip |y| ≤ 1 between x = 2 and the rim"
tex = '400 + 2\left(36\pi - s\right) + 2.25\pi,\quad s = \sqrt{35} + 36\arcsin\tfrac16 - 4'

[area]
value = 980.4685668780503
exact = "the plate's bottom, sides and top less the disk's footprint, 400 + 80 + 400 − (36π − s); the disk's rim 12(2π − 2 arcsin(1/6)), the slot's two sides 4(√35 − 2) and its end 4, the disk's top 36π − s − 2.25π; the small disk's side 3π and top 2.25π"
tex = '\left(880 - 36\pi + s\right) + 12\left(2\pi - 2\arcsin\tfrac16\right) + 4\left(\sqrt{35} - 2\right) + 4 + \left(36\pi - s - 2.25\pi\right) + 3\pi + 2.25\pi'
+++

A square plate with a slotted disk and a small disk stacked on it, one solid.

The plate is the box from (−10, −10, 0) to (10, 10, 1). On it stands a disk of radius 6 about the vertical axis through the origin, from z = 1 to z = 3, with a slot cut through its whole thickness from its rim: the slot is the strip |y| ≤ 1 with x ≥ 2, which runs out through the rim at x = √35 ≈ 5.92 and has a flat end at x = 2. The slot is cut only in the disk, so the plate's top shows in its floor. On top of the disk is a small disk of radius 1.5, centered on the axis, from z = 3 to z = 4. It does not reach the slot (its edge is at x = 1.5).

**As a set:**

$$\{\,|x| \le 10,\ |y| \le 10,\ 0 \le z \le 1\,\} \cup \bigl(\{\,\rho \le 6,\ 1 \le z \le 3\,\} \setminus \{\,x \ge 2,\ |y| \le 1\,\}\bigr) \cup \{\,\rho \le 1.5,\ 3 \le z \le 4\,\}$$
