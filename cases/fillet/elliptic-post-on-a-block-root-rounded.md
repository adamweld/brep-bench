+++
title = "An elliptic post standing on a block, its root rounded"
tags = ["fillet", "concave", "elliptic-cylinder", "analytic"]
origin = "OCCT tests/blend/bfuseblend/A7"

[volume]
value = 39445.22936108748
exact = "36000 + 1080π (the block and the post, π·12·6·15) + L(1 − π/4)·2² + 2π·2³(5/6 − π/4) (the round: its section, a corner 2 by 2 less a quarter disk, carried round the ellipse, whose perimeter L = 48·E(k² = 3/4) = 58.130689323286057 and whose curvature integrates to 2π; Steiner) = 36000 + 1080π + L(4 − π) + 40π/3 − 4π²"
tex = '36000 + 1080\pi + L(4 - \pi) + \tfrac{40\pi}{3} - 4\pi^2,\quad L = 48\,E\!\left(k^2 = \tfrac34\right)'

[area]
value = 10423.839834843573
exact = "6000 (the block's bottom and sides) + (3600 − 72π − 2L − 4π) (its top outside the round's foot, a band 2 wide round the ellipse of area 2L + 4π) + (15L − 2L) (the post's wall above the round) + 72π (its top) + the round, πL + 4π(π − 2) (the quarter circle of length π carried round the ellipse, with Steiner's curvature term): in all 9600 + L(11 + π) + 4π² − 12π"
tex = '9600 + L(11 + \pi) + 4\pi^2 - 12\pi,\quad L = 48\,E\!\left(k^2 = \tfrac34\right)'
+++

A block from (−30, −30, 0) to (30, 30, 10), and standing on its top face an elliptic post: the elliptic cylinder x²/12² + y²/6² ≤ 1 from z = 10 to z = 25, with a flat top. The two are one solid.

Round the ellipse where the post meets the block's top, to radius 2. The round is concave, a ball of radius 2 rolling round the root touching the top face and the post's wall; its section square to the ellipse is the corner 2 by 2 between the top face and the wall less the quarter disk of radius 2, so it meets the top face on the curve 2 out from the ellipse (the ellipse's parallel curve) and the wall 2 above the block, at z = 12. The post's smallest radius of curvature is 6²/12 = 3, so the round does not fold at the ends of the short axis.

**As a set:** with $t$ the distance from the point $(x, y)$ to the ellipse $x^2/12^2 + y^2/6^2 = 1$, the post is $\{x^2/144 + y^2/36 \le 1,\ 10 \le z \le 25\}$ and the round adds $\{x^2/144 + y^2/36 > 1,\ t < 2,\ 0 \le z - 10 < 2,\ (t - 2)^2 + (z - 12)^2 > 2^2\}$ to the block.
