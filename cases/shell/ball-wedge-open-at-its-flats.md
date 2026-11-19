+++
title = "Three quarters of a cut ball shelled inward, open at both its flats"
tags = ["shell", "inward", "sphere", "pole", "concave", "analytic"]
origin = "OCCT tests/offset/faces_type_i/C4"

[volume]
value = 182.212373908208
exact = "(3/4)(B(5, 3) − B(4, 2)) = (3/4)(448π/3 − 216π/3) = 58π, B(R, h) = 4πR³/3 − π(R − h)²(2R + h)/3 the ball of radius R below z = h"
tex = '\tfrac34\bigl(B(5, 3) - B(4, 2)\bigr) = 58\pi,\quad B(R, h) = \tfrac{4}{3}\pi R^3 - \tfrac{1}{3}\pi (R - h)^2 (2R + h)'

[area]
value = 394.4852514911437
exact = "(3/4)(2π·5·8 + 2π·4·6 + 16π + 12π) + 2(D(5, 3) − D(4, 2)) = 394π/3 + 12 − 4√3 − 25 acos(3/5), D(R, h) = (πR² − R² acos(h/R) + h√(R² − h²))/2 the half disk of radius R below z = h: the two spheres, the top, the cavity's flat and the two rims"
tex = '\tfrac34\left(80\pi + 48\pi + 16\pi + 12\pi\right) + 2\bigl(D(5, 3) - D(4, 2)\bigr) = \tfrac{394\pi}{3} + 12 - 4\sqrt3 - 25\arccos\tfrac35'
+++

Three quarters of the ball of radius 5 centered on the origin, cut flat at z = 3: the part with
z ≤ 3 whose angle about the z axis, φ, measured from +x toward +y, lies between 0° and 270°. It
runs from the south pole, (0, 0, −5), which is on the z axis, up to the plane z = 3. Its faces are
the sphere, the flat top (three quarters of the disk of radius 4 at z = 3), and two flat faces in
the half-plane y = 0, x ≥ 0 (φ = 0°) and in the half-plane x = 0, y ≤ 0 (φ = 270°), which meet
along the z axis at a concave edge.

Shell it inward to a wall 1 thick, open at both faces in the half-planes. The outside of the wall
is the sphere and the top, unmoved, and the inside is each of them moved 1 inward along its
normal: the sphere onto the sphere of radius 4 about the origin, and the top onto the plane
z = 2. The moved faces are trimmed where they meet each other, and run on until they meet the
surfaces of the openings, the two half-planes, which are left as rims between the outside and the
inside.

So the result is the cut ball's three quarters less the cavity, the part of the ball of radius 4
below z = 2 with φ between 0° and 270°. The cavity's flat is three quarters of the disk of radius
√12 at z = 2. Each rim is the part of its half-plane between the two: inside the circle of radius
5 and below z = 3, outside the circle of radius 4 or above z = 2. The two rims meet along the z
axis from (0, 0, −5) to (0, 0, −4) and from (0, 0, 2) to (0, 0, 3).

**As a set:**

$$\{\, \lVert p \rVert \le 5,\ z \le 3,\ 0^\circ \le \varphi \le 270^\circ \,\} \setminus \{\, \lVert p \rVert \le 4,\ z \le 2 \,\}$$
