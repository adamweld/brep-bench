+++
title = "A cube united with a wide frustum standing beside it, cutting across its side"
tags = ["boolean", "union", "analytic", "cone", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/ZH1"

[volume]
value = 1081.917485049346
exact = "V = 6³ + 296π − C, 296π the frustum (π·6/3·(64 + 48 + 36)), C its part inside the cube. About the axis (3, −5) the radius at height z is ρ = 8 − z/3, and the cube takes of each level the region 0 ≤ y ≤ −5 + √(ρ² − u²), u = x − 3, |u| ≤ 3, of area 3√(ρ² − 9) + ρ² asin(3/ρ) − 30. Integrated over z: C = −108√3 − 180 − 36π + 27Λ + 512 asin(3/8) + 48√55, Λ = ln(3(2 + √3)/(8 + √55))"
tex = '216 + 296\pi - \left[-108\sqrt3 - 180 - 36\pi + 27\Lambda + 512\arcsin\tfrac38 + 48\sqrt{55}\right],\quad \Lambda = \ln\tfrac{3(2+\sqrt3)}{8+\sqrt{55}}'

[area]
value = 695.9675133481583
exact = "area = (36 + 64π − B) (bottom: the cube's face and the frustum's base disc, B = 3√55 + 64 asin(3/8) − 30 where they overlap) + (36 + 36π − T) (top, T = 9√3 + 6π − 30 where they overlap) + 0 (face y = 0, wholly inside the frustum) + 2(36 − F) (faces x = 0, 6, F = −27√3 − 30 + (27/2)Λ + 12√55 inside the frustum) + 36 (face y = 6) + 28π√10 − √10(−6π − 9√3 + 3√55 + 64 asin(3/8)) (the cone's wall, 28π√10 in all, less the part inside the cube)"
tex = '(36 + 64\pi - B) + (36 + 36\pi - T) + 2\,(36 - F) + 36 + 28\pi\sqrt{10} - \sqrt{10}\left(-6\pi - 9\sqrt3 + 3\sqrt{55} + 64\arcsin\tfrac38\right)'
+++

A cube of edge 6 from (0, 0, 0) to (6, 6, 6), and a cone frustum on the vertical axis through (3, −5), standing beside the cube's side face y = 0, 5 away from it: base radius 8 at z = 0, top radius 6 at z = 6, so its ends lie in the cube's bottom and top planes. Make their union.

The frustum's radius is larger than the cube's edge at every height, and its wall cuts across a strip of the cube: it takes the whole face y = 0 and, from the faces x = 0 and x = 6, a strip bounded by the wall, √55 − 5 (about 2.4) deep at the bottom and 3√3 − 5 (about 0.2) deep at the top, measured from the face y = 0 at its edge. The frustum reaches no farther than y = 3. The result is one solid: the cube with the frustum's body covering its front.

**As a set:**

$$[0,6]^3 \cup \{(x-3)^2 + (y+5)^2 \le (8 - z/3)^2,\ 0 \le z \le 6\}$$
