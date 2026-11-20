+++
title = "A bar with a drafted side and rounded top edges, notched by a rounded pocket"
tags = ["feature", "pocket", "closed-profile", "draft", "fillet", "cylinder", "analytic"]
origin = "OCCT feat/featlf/E1, OCCT feat/featlf/E2"

[volume]
value = 5395.0317533718288
exact = "30·A − (3·(12 + 9π/2) + 10·S₁₁), S₁₁ = S − 33 + (4 − π), with A, S, r, xt, xr, θ as in the groove case: r = 2, xt = 20 − 10 tan 10°, xr = 20 − 7 tan 10°, θ = 100°"
tex = '30A - \left(3\left(12 + \tfrac{9\pi}{2}\right) + 10S_{11}\right),\quad S_{11} = S - 33 + (4 - \pi)'

[area]
value = 2153.2039038116323
exact = "A_bar − removed + floor + walls: removed = (12 + 9π/2) + 10(ℓ − 7); floor = (12 + 9π/2) + 10(xr − 11); walls = 3(4 + 3π) + 2·S₁₁; A_bar = 2A + 30P and ℓ as in the groove case"
tex = '2A + 30P - \left[\left(12 + \tfrac{9\pi}{2}\right) + 10(\ell - 7)\right] + \left[\left(12 + \tfrac{9\pi}{2}\right) + 10(x_r - 11)\right] + \left[3(4 + 3\pi) + 2S_{11}\right]'
+++

A bar with a drafted side and rounded top edges, and a pocket with rounded corners cut into it from
the top, opening out through the drafted side.

**The bar.** Its cross-section, in the xz plane, is the quadrilateral with corners (0, 0), (20, 0),
(20 − 10·tan 10°, 10) and (0, 10): the side at x = 20 is drafted, leaning in 10° from the
vertical as it rises, and the side at x = 0 is vertical. The cross-section is extruded along y from
0 to 30, and the bar's ends, at y = 0 and y = 30, are flat. The bar's two long top edges, the one at
x = 0 and the one where the drafted side meets the top, are rounded with circular arcs of radius 2,
each tangent to the top and to its side, running right through to the end faces. Its other edges
are sharp.

**The pocket.** The rectangle 8 ≤ x ≤ 30, 10 ≤ y ≤ 20 of the xy plane with its four corners
rounded by arcs of radius 3 (centres at (11, 13), (11, 17), (27, 13) and (27, 17)), cut straight
down from the top to the flat floor z = 7, with vertical walls. Its end at x = 30 lies outside the
bar, so the pocket opens through the drafted side and its round; its walls there stop on that side.
Its rounded inner end, at x = 8, lies on the flat top.
