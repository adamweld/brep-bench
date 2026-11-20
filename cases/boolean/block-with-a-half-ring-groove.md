+++
title = "A block with a half ring cut through it from its face"
tags = ["boolean", "cut", "analytic", "revolve", "torus", "cylinder"]
origin = "OCCT boolean/bcut_simple/G6"

[volume]
value = 2536.9868696759004
exact = "24·12·12 − πRA, with A = 2rL + πr² = 24 + 4π, R = 8"
tex = '24\cdot 12\cdot 12 - \pi R A,\quad A = 24 + 4\pi'

[area]
value = 1984.2874943507613
exact = "1440 − 2A + πRP, with P = 2L + 2πr = 12 + 4π"
tex = '1440 - 2A + \pi R P,\quad P = 12 + 4\pi'
+++

A block from (−12, 0, 0) to (12, 12, 12), and a ring made by turning a slot-shaped (stadium) profile about the z axis. The profile lies in the plane y = 0, which is the block's face y = 0, on the side x > 0: it is the stadium of half-width 2 whose two straight sides are the lines x = 6 and x = 10 from z = 3 to z = 9, closed by a half disk of radius 2 at each end, centred at (8, 3) and (8, 9). The ring is the solid it sweeps in a whole turn: between the cylinders of radius 6 and 10 about the z axis, with a rounded top and bottom made of tori of tube radius 2 and ring radius 8. Cut the ring from the block.

Only the half of the ring with y ≥ 0 meets the block, and it lies wholly inside it but for the face y = 0, where its profile cuts two stadium-shaped openings, one at x > 0 and one at x < 0. What is left is the block with an arched tunnel in the shape of half a ring, running from one opening to the other.

**As a set:**

$$[-12,12]\times[0,12]\times[0,12] \setminus \{\,y \ge 0,\ (|\rho - 8| \le 2 \wedge |z - 6| \le 3) \vee (\rho - 8)^2 + (z - 3)^2 \le 4 \vee (\rho - 8)^2 + (z - 9)^2 \le 4\,\}$$
