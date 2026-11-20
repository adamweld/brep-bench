+++
title = "A cone cored from the top over a tube"
tags = ["boolean", "cut", "cone", "cylinder", "analytic"]
origin = "OCCT boolean/gdml_public/C6"

[volume]
value = 373.8495257771854
exact = "π·6/3·(6² + 6·4 + 4²) − π3²·2 − π(3² − 2²)·3 = 152π − 18π − 15π"
tex = '\tfrac{\pi\cdot 6}{3}\left(6^2 + 6\cdot 4 + 4^2\right) - \pi\cdot 3^2\cdot 2 - \pi\left(3^2 - 2^2\right)\cdot 3 = 152\pi - 18\pi - 15\pi = 119\pi'

[area]
value = 494.0014747533626
exact = "side of the frustum π(6 + 4)√(6² + 2²) = 20√10π, bottom 36π, top ring π(4² − 3²), pocket wall 2π·3·2, pin top π·2², groove outer wall 2π·3·3, pin side 2π·2·3, groove floor π(3² − 2²): 94π + 20√10π"
tex = '20\sqrt{10}\,\pi + 36\pi + 7\pi + 12\pi + 4\pi + 18\pi + 12\pi + 5\pi = 94\pi + 20\sqrt{10}\,\pi'
+++

A cone frustum on the z axis from radius 6 at z = 0 to radius 4 at z = 6, so that its side is ρ = 6 − z/3, with a cutter taken out of it. The cutter is a cylinder of radius 3 from z = 4 up past the top, fused to a tube between radii 2 and 3 from z = 1 to z = 4. They share the plane z = 4, where the tube's top ring is the cylinder's floor.

What is left is a cup, open at the top, with a pin standing in it: a pocket of radius 3 from the top down to z = 4, below its floor a groove between radii 2 and 3 down to z = 1, so that the pin of radius 2 reaches from the solid base below z = 1 up to a flat top at z = 4.

**As a set:**

$$\{\,\rho \le 6 - \tfrac{z}{3},\ 0 \le z \le 6\,\} \setminus \bigl(\{\,\rho \le 3,\ z \ge 4\,\} \cup \{\,2 \le \rho \le 3,\ 1 \le z \le 4\,\}\bigr)$$
