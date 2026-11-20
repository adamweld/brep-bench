+++
title = "A block less an oblate spheroid tilted about its centre in the top face"
tags = ["boolean", "cut", "spheroid", "box", "tilted", "analytic"]
origin = "OCCT tests/bugs/modalg_2/bug4717_4, OCCT tests/bugs/modalg_2/bug4717_5"

[volume]
value = 2332.979356723418
exact = "2400 − 64π/3: the block less the half of the spheroid below its top face"
tex = '2400 - \tfrac{64\pi}{3}'

[area]
value = 1311.3779284516013
exact = "1280 − 32π/√7 + 16π(1 + ln(2 + √3)/(2√3)): the block's 1280, less the section ellipse its top face loses, plus the half of the spheroid's surface below it, now the wall of the dimple"
tex = '1280 - \tfrac{32\pi}{\sqrt 7} + 16\pi\left(1 + \tfrac{\ln(2 + \sqrt 3)}{2\sqrt 3}\right)'
+++

A block, the box −10 ≤ x ≤ 10, −10 ≤ y ≤ 10, −6 ≤ z ≤ 0, and an oblate spheroid whose centre is the
origin, in the block's top face z = 0. The spheroid is the ellipse of semi-axes 4 and 2 turned a
whole turn about its short axis: radius 4 about the axis and 2 along it. The axis is the z axis
tilted 30° about the x axis toward +y, the direction (0, 1/2, √3/2). With u = (y + √3 z)/2 the
distance along the axis, the spheroid is { (x² + y² + z² − u²)/16 + u²/4 ≤ 1 }.

The top face passes through the spheroid's centre and cuts it in an ellipse.
The half of the spheroid below the top face lies wholly inside the block, its deepest point
√7 ≈ 2.65 below z = 0 and its reach to the side at most 4; the half above stands clear of the
block.

Cut the spheroid out of the block: the block with a dimple pressed into its top, the lower half
of the tilted spheroid, its rim the section ellipse in the top face.

**As a set:**

$$\{\,-10 \le x \le 10,\ -10 \le y \le 10,\ -6 \le z \le 0\,\} \setminus \{\,\tfrac{x^2 + y^2 + z^2 - u^2}{16} + \tfrac{u^2}{4} \le 1\,\}$$
