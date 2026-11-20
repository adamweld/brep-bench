+++
title = "A cube raised on a tapered riser up to a slab"
tags = ["draft", "taper", "extrude", "up-to", "boolean", "analytic"]
origin = "OCCT bugs/modalg_7/bug30150"

[volume]
value = 2715.057412611072
exact = "1000 + (6/3)(a² + ab + b²) + 24²·2, a = 10, b = 10 − 12·tan 3°: the cube, the riser (a frustum of height 6) and the slab"
tex = '1000 + \tfrac{6}{3}\left(a^2 + ab + b^2\right) + 24^2\cdot 2,\quad a = 10,\ b = 10 - 12\tan 3^\circ'

[area]
value = 1988.9546462919554
exact = "500 (floor and four walls) + 4·(a + b)/2·6/cos 3° (the riser's sides) + the slab's 576 (top) + 192 (sides) + (576 − b²) (underside, less the riser's top)"
tex = '500 + 4\cdot\tfrac{a + b}{2}\cdot\tfrac{6}{\cos 3^\circ} + 576 + 192 + \left(576 - b^2\right)'
+++

A cube with a tapered riser from its open side up to a slab: the extrusion of an open cube, drafted, until it meets the slab.

The cube is the box −5 ≤ x, y ≤ 5, 0 ≤ z ≤ 10. Its five faces are its floor z = 0 and four walls; the sixth side, the square at z = 10, is the opening, and it is the side that faces the slab, which lies above: the box −12 ≤ x, y ≤ 12, 16 ≤ z ≤ 18. (An extrusion up to the slab can only rise from the opening: the walls lie along the direction of rise, so extruding them that way sweeps out no volume.) The opening is extruded in the direction +z with a taper of 3°, each side of the riser leaning inward by 3° from the vertical as it rises, and stops at the slab's underside z = 16, where it joins the slab. The riser's side faces are four planes through the opening's four edges; at height z its section is the square |x|, |y| ≤ 5 − (z − 10)·tan 3°, so its top at z = 16 is a square of side 10 − 12·tan 3° ≈ 9.371. The cube, the riser and the slab are united into one solid: the cube's walls and floor are as they were, and the opening is no longer a face, being the riser's foot.

**As a set:**

$$\{|x|, |y| \le 5,\ 0 \le z \le 10\}\ \cup\ \{\,|x|, |y| \le 5 - (z - 10)\tan 3^\circ,\ 10 \le z \le 16\,\}\ \cup\ \{|x|, |y| \le 12,\ 16 \le z \le 18\}$$
