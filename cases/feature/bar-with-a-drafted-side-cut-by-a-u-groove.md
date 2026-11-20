+++
title = "A bar with a drafted side and rounded top edges, cut by a U-shaped groove"
tags = ["feature", "groove", "open-profile", "draft", "fillet", "cylinder", "analytic"]
origin = "OCCT feat/featlf/E3, OCCT feat/featlf/E4, OCCT feat/featlf/E5"

[volume]
value = 5327.2024152769549
exact = "30·A − (18·3·3 + 6·S₇), A = (20 + xt)/2·10 − r²(1 − π/4) − r²(cot(θ/2) − (π − θ)/2), S₇ = S − (7·3 − r²(1 − π/4)), S = (xr + xt)/2·3 − r²(1 − π/4) − r²(cot(θ/2) − (π − θ)/2); r = 2, xt = 20 − 10 tan 10°, xr = 20 − 7 tan 10°, θ = 100°"
tex = '30A - \left(18\cdot 3\cdot 3 + 6S_7\right),\quad A = \tfrac{20 + x_t}{2}\cdot 10 - r^2\left(1 - \tfrac{\pi}{4}\right) - r^2\left(\cot\tfrac{\theta}{2} - \tfrac{\pi - \theta}{2}\right)'

[area]
value = 2320.6228591582106
exact = "A_bar − removed + floor + walls: A_bar = 2A + 30·P, P = 20 + 10 + xt + 10/cos 10° − 2r·(1 + cot(θ/2)) + r(π/2) + r(π − θ); removed = 18·3 + 6(ℓ − 3), ℓ = (xt − 4 − d) + r(π − θ) + (3/cos 10° − d), d = r·cot(θ/2); floor = 18·3 + 6(xr − 7); walls = 108 + 4·S₇"
tex = '2A + 30P - \left[18\cdot 3 + 6(\ell - 3)\right] + \left[18\cdot 3 + 6(x_r - 7)\right] + \left[108 + 4S_7\right]'
+++

A bar, and a groove cut across it in the shape of an open U.

**The bar.** Its cross-section, in the xz plane, is the quadrilateral with corners (0, 0), (20, 0),
(20 − 10·tan 10°, 10) and (0, 10): the side at x = 20 is drafted, leaning in 10° from the
vertical as it rises, and the side at x = 0 is vertical. The cross-section is extruded along y from
0 to 30, and the bar's ends, at y = 0 and y = 30, are flat. The bar's two long top edges, the one at
x = 0 and the one where the drafted side meets the top, are rounded with circular arcs of radius 2,
each tangent to the top and to its side, running right through to the end faces. Its other edges
are sharp.

**The groove.** A groove 3 wide, cut down from the top to the flat floor z = 7, with vertical walls.
Its centre line is an open U of three straight segments: from (30, 7.5) in to (5.5, 7.5), across to
(5.5, 22.5), and out to (30, 22.5). The two arms run on past the drafted side to x = 30, outside the
bar, so the groove opens through that side and its round. Its corners are square. The groove cuts
across the drafted side and the round beside it, and what the U encloses, between y = 9 and
y = 21 and beyond x = 7, is left standing as a tongue with the top and round of the bar and the
floor beside it.

**As a set:** with K the bar and G the region of the xy plane that is the union of
$[4,30]\times[6,9]$, $[4,30]\times[21,24]$ and $[4,7]\times[6,24]$,

$$K \setminus \bigl(G \times [7, \infty)\bigr)$$
