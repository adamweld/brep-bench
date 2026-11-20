+++
title = "A cube less a frustum centred on a side face, spilling past its edges"
tags = ["boolean", "cut", "analytic", "coincident", "notch"]
origin = "OCCT tests/boolean/bopcut_simple/ZG1"

[volume]
value = 132.26723354703967
exact = "6³ less the frustum's part inside the cube. At height z the frustum's radius is ρ(z) = 4 − z/3 about the axis (3, 0); the cube takes the half disk y ≥ 0, less for ρ > 3 the circular segment ρ² acos(3/ρ) − 3√(ρ² − 9) past the faces x = 0 and x = 6. Integrated over z: 28π + 24√7 − 64 acos(3/4) + 27 ln((4 − √7)/3)"
tex = '216 - \left[28\pi + 24\sqrt7 - 64\arccos\tfrac34 + 27\ln\tfrac{4-\sqrt7}{3}\right]'

[area]
value = 193.0759677494088
exact = "216, less the openings in the bottom (8π − 16 acos(3/4) + 3√7), the top (2π), the face y = 0 (33) and the two faces x = 0, 6 (each 6√7 + (27/2)ln((4 − √7)/3)), plus the cone's wall in the cube, √10(3√7 + 6π − 16 acos(3/4)), which is the integral of ρ√(1 + ρ'²) over the arc of the circle inside the cube"
tex = '216 - \left(8\pi - 16\arccos\tfrac34 + 3\sqrt7\right) - 2\pi - 33 - 2F + \sqrt{10}\left(3\sqrt7 + 6\pi - 16\arccos\tfrac34\right),\quad F = 6\sqrt7 + \tfrac{27}{2}\ln\tfrac{4-\sqrt7}{3}'
+++

A cube of edge 6 from (0, 0, 0) to (6, 6, 6), and a cone frustum standing on the plane z = 0 on the vertical axis through (3, 0), the middle of the cube's side face y = 0: base radius 4 at z = 0, top radius 2 at z = 6, so its ends lie in the cube's bottom and top planes. Cut the frustum away from the cube.

Half of the frustum is inside the cube. Its base radius, 4, exceeds half the face's width, 3, so low down, below z = 3, its circle spills past the cube's side faces x = 0 and x = 6; above z = 3 the circle's radius is under 3 and it lies within the face's width. The cone's wall therefore cuts the faces x = 0 and x = 6 as well as the face y = 0, and the notch runs up the side as a half cone.

**As a set:**

$$[0,6]^3 \setminus \{(x-3)^2 + y^2 \le (4 - z/3)^2\}$$
