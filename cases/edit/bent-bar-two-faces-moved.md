+++
title = "A bent triangular bar, two of its faces moved out by 2"
tags = ["edit", "move-face", "revolution", "cone", "cylinder", "analytic"]
origin = "OCCT tests/bugs/modalg_2/bug427_6"

[volume]
value = 1140.2817779696286
exact = "(π/2)·(196/3)·(100/9) = 9800π/27: a quarter turn of the triangle of area 196/3 whose centroid is 100/9 from the axis (Pappus)"
tex = '\tfrac{\pi}{2}\cdot\tfrac{196}{3}\cdot\tfrac{100}{9} = \tfrac{9800\pi}{27}'

[area]
value = 827.0797903655549
exact = "(π/4)((52/3)² − 8²) + (π/2)·8·14 + (π/2)(38/3)(14√13/3) + 2·196/3 = π(1036 + 266√13)/9 + 392/3: the bottom, the inner cylinder, the cone and the two end triangles"
tex = '\tfrac{\pi}{4}\left(\left(\tfrac{52}{3}\right)^2 - 8^2\right) + \tfrac{\pi}{2}\cdot 8\cdot 14 + \tfrac{\pi}{2}\cdot\tfrac{38}{3}\cdot\tfrac{14\sqrt{13}}{3} + 2\cdot\tfrac{196}{3} = \tfrac{\pi(1036 + 266\sqrt{13})}{9} + \tfrac{392}{3}'
+++

A bent bar: the right triangle with corners (ρ, z) = (10, 0), (16, 0), (10, 9), in the half plane
y = 0, x > 0, swept along the arc of radius 10 about the z axis at its foot, a quarter turn from
the plane y = 0 to the plane x = 0 toward +y. It has five faces: the bottom, z = 0, a quarter
annulus between radii 10 and 16; the inner side, the cylinder of radius 10 about the z axis, from
z = 0 to z = 9; the slanting side, the cone ρ = 16 − 2z/3 through the other two corners of the
triangle; and the two triangular ends, in the planes y = 0 and x = 0.

Then edit the bar directly: move two faces out, each by 2 along its outward normal. The inner
cylinder moves toward the axis, to the cylinder of radius 8, and the bottom moves down, to the
plane z = −2. The faces beside them follow: the cone runs on, down and in, to meet both moved
faces, and the two end planes are reshaped to match, staying in their planes.

The result is the bent bar whose triangle is the right triangle with corners (8, −2),
(52/3, −2) and (8, 12), swept the same quarter turn about the z axis: the cone ρ = 16 − 2z/3
reaches z = −2 at ρ = 52/3 and ρ = 8 at z = 12.

**As a set:**

$$\{\,\rho \ge 8,\ z \ge -2,\ \rho \le 16 - \tfrac{2z}{3},\ x \ge 0,\ y \ge 0\,\}$$
