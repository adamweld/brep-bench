+++
title = "A block standing out of a cylinder's side, its flanks drafted"
tags = ["draft", "planar", "cylinder", "boolean", "analytic"]
origin = "OCCT tests/draft/angle/D7"

[volume]
value = 6596.527032183003
exact = "2000π + ∫₅¹⁵ A(w(z)) dz = 2000π + [G(3) − G(3 − 10t)]/t, t = tan 5°, w(z) = 3 − (z − 5)t, A(w) = 32w − w√(100 − w²) − 100 asin(w/10) (the block's section at height z, from the cylinder out to x = 16, 2w wide), G(w) = 16w² + (100 − w²)^{3/2}/3 − 100(w·asin(w/10) + √(100 − w²))"
tex = '2000\pi + \int_5^{15} A(w(z))\,dz,\quad w(z) = 3 - (z - 5)t,\ t = \tan 5^\circ,\ A(w) = 32w - w\sqrt{100 - w^2} - 100\arcsin\tfrac{w}{10}'

[area]
value = 2074.328342036844
exact = "600π − ∫₅¹⁵ 20 asin(w/10) dz + A(w(5)) + A(w(15)) + ∫₅¹⁵ 2w dz + 2√(1 + t²)·∫₅¹⁵ (16 − √(100 − w²)) dz, w and t as for the volume: the cylinder's floor, top and side less where the block leaves it, the block's floor and top, its end face x = 16, and its two flanks"
tex = '600\pi - \int_5^{15} 20\arcsin\tfrac{w}{10}\,dz + A(w(5)) + A(w(15)) + \int_5^{15} 2w\,dz + 2\sqrt{1 + t^2}\int_5^{15}\left(16 - \sqrt{100 - w^2}\right)dz'
+++

A cylinder of radius 10 about the z axis, from z = 0 to z = 20, and on its side a block, from
x = 0 to x = 16, y = −3 to y = 3 and z = 5 to z = 15, united with it. The block stands out of the
cylinder's wall; its part inside the cylinder is part of the cylinder.

Draft the block's two flanks, its faces y = 3 and y = −3, by 5°, with the pull direction +z and the
neutral plane z = 5, the block's floor: each turns about its line at z = 5 until it leans 5° from the
z direction, inward as it rises, y = ±(3 − (z − 5)·tan 5°). The block's end face x = 16, its floor
and its top are not drafted, nor is the cylinder; the top is trimmed to the flanks, and each flank
runs from the cylinder's wall, where it ends on a curve, out to x = 16.

**As a set:**

$$\{\,\rho \le 10,\ 0 \le z \le 20\,\} \cup \{\,0 \le x \le 16,\ |y| \le 3 - (z-5)\tan 5^\circ,\ 5 \le z \le 15\,\}$$
