+++
title = "A cube united with a frustum centred on a side face, spilling past its edges"
tags = ["boolean", "union", "analytic", "cone", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/ZG1"

[volume]
value = 308.1964221480681
exact = "V = 6³ + 56π − C, 56π the frustum (π·6/3·(16 + 8 + 4)), C its part inside the cube. At height z the radius is ρ(z) = 4 − z/3 about the axis (3, 0); the cube takes the half disk y ≥ 0, less for ρ > 3 the circular segment ρ² acos(3/ρ) − 3√(ρ² − 9) past the faces x = 0 and x = 6. Integrated over z: C = 28π + 24√7 − 64 acos(3/4) + 27 ln((4 − √7)/3)"
tex = '216 + 56\pi - \left[28\pi + 24\sqrt7 - 64\arccos\tfrac34 + 27\ln\tfrac{4-\sqrt7}{3}\right]'

[area]
value = 278.8437829443269
exact = "area = (36 + 16π − B) (bottom: the cube's face and the frustum's base disc, B = 8π − 16 acos(3/4) + 3√7 where they overlap) + (36 + 4π − 2π) (top, the overlap a half disc of the top disc) + (36 − 33) (face y = 0, the part the frustum does not cover) + 2(36 − F) (faces x = 0, 6, F = 6√7 + (27/2) ln((4 − √7)/3) covered) + 36 (face y = 6) + 12π√10 − √10(3√7 + 6π − 16 acos(3/4)) (the cone's wall, 12π√10 in all, less the √10(…) inside the cube)"
tex = '(36 + 16\pi - B) + (36 + 2\pi) + 3 + 2\,(36 - F) + 36 + 12\pi\sqrt{10} - \sqrt{10}\left(3\sqrt7 + 6\pi - 16\arccos\tfrac34\right),\quad B = 8\pi - 16\arccos\tfrac34 + 3\sqrt7,\ F = 6\sqrt7 + \tfrac{27}{2}\ln\tfrac{4-\sqrt7}{3}'
+++

A cube of edge 6 from (0, 0, 0) to (6, 6, 6), and a cone frustum standing on the plane z = 0 on the vertical axis through (3, 0), the middle of the cube's side face y = 0: base radius 4 at z = 0, top radius 2 at z = 6, so its ends lie in the cube's bottom and top planes. Make their union.

Half of the frustum is inside the cube. Its base radius, 4, exceeds half the face's width, 3, so low down, below z = 3, its circle spills past the cube's side faces x = 0 and x = 6; above z = 3 the circle's radius is under 3 and it lies within the face's width. The result is the cube with a half cone against its side, which also reaches round to the faces x = 0 and x = 6 low down.

**As a set:**

$$[0,6]^3 \cup \{(x-3)^2 + y^2 \le (4 - z/3)^2,\ 0 \le z \le 6\}$$
