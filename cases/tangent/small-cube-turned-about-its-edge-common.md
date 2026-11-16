+++
title = "A small cube and itself turned 1.2° about one of its edges, their common part"
tags = ["tangent", "boolean", "intersection", "planar", "sliver", "small"]

[volume]
value = 9.7927235072578411e-10
exact = "s³(1 − sin θ)/cos θ, s = 0.001, θ = 1.2°"
tex = '\frac{s^3(1 - \sin\theta)}{\cos\theta},\quad s = 0.001,\ \theta = 1.2^\circ'

[area]
value = 5.9170894029031365e-6
exact = "2s² + 4s²(1 − sin θ)/cos θ"
tex = '2s^2 + \frac{4s^2(1 - \sin\theta)}{\cos\theta}'
+++

A small cube, of side 0.001, from the origin to (0.001, 0.001, 0.001), and a copy of itself
turned 1.2° about the z axis, the line of one of the cube's own edges: the part common to both.

The common part is a prism 0.001 high over the part common to the two squares, a quadrilateral
with corners (0, 0), s(cos θ, sin θ), (s(1 − sin θ)/cos θ, s) and (0, s): the copy's side
through the origin cuts a thin wedge off the cube, and its far side cuts off the corner
opposite. Its area is s²(1 − sin θ)/cos θ and its perimeter 2s + 2s(1 − sin θ)/cos θ. The turn
moves the cube's far corners by 0.000021 and 0.000030, small distances, but the common part is
2.1% less than the cube: a kernel that takes the turned copy's sides for the cube's own, by a
tolerance that does not shrink with the model, returns the cube and is 2.1% out.
