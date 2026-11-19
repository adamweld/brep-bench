+++
title = "A post with a collar open at its seam"
tags = ["tangent", "boolean", "union", "analytic", "coincident", "cylinder", "seam"]
origin = "OCCT tests/boolean/bfuse_simple/E4"

[volume]
value = 1979.2033717615698
exact = "πR²H + (θ/2)(R₂² − R²)h = 500π + (5π/6)(64 − 25)·4 = 630π, R = 5, H = 20, R₂ = 8, h = 4, θ = 5π/3 the collar's turn"
tex = '\pi R^2 H + \tfrac{\theta}{2}\left(R_2^2 - R^2\right) h = 630\pi,\quad R = 5,\ H = 20,\ R_2 = 8,\ h = 4,\ \theta = \tfrac{5\pi}{3}'

[area]
value = 1076.4335389525806
exact = "2πRH − θRh + 2πR² + θR₂h + θ(R₂² − R²) + 2(R₂ − R)h = 200π − 100π/3 + 50π + 160π/3 + 65π + 24 = 335π + 24: the post's wall less the collar's bore, its ends, and the collar's outside, top and bottom, and two flat ends"
tex = '2\pi R H - \theta R h + 2\pi R^2 + \theta R_2 h + \theta\left(R_2^2 - R^2\right) + 2(R_2 - R)h = 335\pi + 24'
+++

A cylinder (a post) of radius 5 on the z axis, from z = 0 to z = 20, united with a collar around it:
the rectangle 5 ≤ ρ ≤ 8, 8 ≤ z ≤ 12 in the half-plane at φ = 30°, turned 300° about the z axis, to
φ = 330°. (φ is the angle about the z axis from +x toward +y, and ρ the distance from it.) The
collar is an open ring with a gap of 60°, centered on the +x side.

The collar's bore is part of the post's own wall: the two cylindrical faces coincide over 300° of
the band 8 ≤ z ≤ 12. The union is the post with the collar on it. In that band the post's wall is
left only in the gap, between φ = −30° and φ = 30°, where the collar's two flat ends run straight
out from it.

This case is about coincident curved faces whose overlap stops at a gap. If your post's cylinder
has a seam, put it in the half-plane φ = 0, in the middle of the gap.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 20\,\} \cup \{\,5 \le \rho \le 8,\ 8 \le z \le 12,\ 30^\circ \le \varphi \le 330^\circ\,\}$$
