+++
title = "A post standing by a rounded rim, its foot rounded across the rim's round"
tags = ["fillet", "torus", "cylinder", "planar", "boss", "tangent", "unsettled"]
origin = "OCCT tests/blend/complex/G9"
settled = false
+++

A short wide cylinder with its rim already rounded, and a post standing near the rim, the post's foot
then rounded where it meets the top and the round together.

- **The cylinder:** radius 10 on the z axis from z = 0 to z = 10, the circular edge around its top
  face rounded to radius 2: the round is a quarter of a torus, centered on the circle of radius 8 at
  z = 8, tangent to the top at ρ = 8 and to the side at z = 8.
- **The post:** the cylinder of radius 3 whose axis is the vertical line through (6.5, 0), from z = 0
  to z = 20, joined to the first. Its wall reaches ρ = 9.5 at its far side, past the top's edge at
  ρ = 8, so there it stands out of the round, not the flat top: the post's wall meets the round where
  it is 0.68 below the top.

Round the foot of the post to radius 0.5: the whole edge where the post's wall meets the flat top and
the round, a closed curve running over the round's tangent line at ρ = 8. It is a concave round, a
ball of radius 0.5 rolling over the top and the round and touching the post's wall, which adds
material.
