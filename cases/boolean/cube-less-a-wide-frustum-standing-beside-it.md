+++
title = "A cube less a wide frustum standing beside it, cutting a strip off its side"
tags = ["boolean", "cut", "analytic", "coincident", "notch"]
origin = "OCCT tests/boolean/bopcut_simple/ZH1"

[volume]
value = 152.00605958676718
exact = "6³ less the frustum's part inside the cube. About the axis (3, −5) the radius at height z is ρ = 8 − z/3, and the cube takes of each level the region 0 ≤ y ≤ −5 + √(ρ² − u²), u = x − 3, |u| ≤ 3, of area 3√(ρ² − 9) + ρ² asin(3/ρ) − 30. Integrated over z: −108√3 − 180 − 36π + 27Λ + 512 asin(3/8) + 48√55, Λ = ln(3(2 + √3)/(8 + √55))"
tex = '216 - \left[-108\sqrt3 - 180 - 36\pi + 27\Lambda + 512\arcsin\tfrac38 + 48\sqrt{55}\right],\quad \Lambda = \ln\tfrac{3(2+\sqrt3)}{8+\sqrt{55}}'

[area]
value = 182.14000437753558
exact = "216, less the openings in the top, 9√3 + 6π − 30, and the bottom, 3√55 + 64 asin(3/8) − 30, the whole face y = 0 (36) and the two faces x = 0, 6 (each −27√3 − 30 + (27/2)Λ + 12√55), plus the cone's wall inside the cube, √10(−6π − 9√3 + 3√55 + 64 asin(3/8)), the integral of ρ√(1 + ρ'²)·2 asin(3/ρ)"
tex = '216 - \left(9\sqrt3 + 6\pi - 30\right) - \left(3\sqrt{55} + 64\arcsin\tfrac38 - 30\right) - 36 - 2F + \sqrt{10}\left(-6\pi - 9\sqrt3 + 3\sqrt{55} + 64\arcsin\tfrac38\right),\quad F = -27\sqrt3 - 30 + \tfrac{27}{2}\Lambda + 12\sqrt{55}'
+++

A cube of edge 6 from (0, 0, 0) to (6, 6, 6), and a cone frustum on the vertical axis through (3, −5), standing beside the cube's side face y = 0, 5 away from it: base radius 8 at z = 0, top radius 6 at z = 6, so its ends lie in the cube's bottom and top planes. Cut the frustum away from the cube.

The frustum's radius is larger than the cube's edge at every height, and its wall cuts across a strip of the cube: it takes the whole face y = 0 and, from the faces x = 0 and x = 6, a strip bounded by the wall, √55 − 5 (about 2.4) deep at the bottom and 3√3 − 5 (about 0.2) deep at the top, measured from the face y = 0 at its edge. What is left of the cube is the block behind the cone's curved wall, a single solid.

**As a set:**

$$[0,6]^3 \setminus \{(x-3)^2 + (y+5)^2 \le (8 - z/3)^2\}$$
