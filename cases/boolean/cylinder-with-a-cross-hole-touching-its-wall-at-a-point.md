+++
title = "A cylinder cross-drilled by a thinner cylinder whose side touches its wall at a point"
tags = ["boolean", "cut", "tangent", "singular", "hole"]
settled = false
origin = "OCCT tests/boolean/bopcut_simple/ZE7"
+++

A cylinder of radius 4 and length 10 on the x axis, from x = −5 to x = 5, and a second cylinder of radius 2 and length 10 whose axis is the line x = 0, z = 2 running in the y direction, from y = −5 to y = 5: the cross hole of a thin cylinder through a thick one, with the thin axis moved up by its own radius, so that the axes are skew. Cut the second away from the first.

The thin cylinder's highest line, x = 0, z = 4, meets the thick cylinder's highest line, y = 0, z = 4, at the single point (0, 0, 4), and there the two walls are tangent, both having the plane z = 4 as their tangent plane. The hole's rim crosses itself at that point. The two walls' intersection is made of elliptic arcs, and the case has no closed form and is unsettled.
