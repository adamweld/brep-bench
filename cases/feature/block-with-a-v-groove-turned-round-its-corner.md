+++
title = "A block with a V-groove turned round its corner from one face to the next"
tags = ["feature", "groove", "revolve", "up-to-face", "cone", "planar", "analytic"]
origin = "OCCT feat/featrevol/A1, OCCT feat/featrevol/A2, OCCT feat/featrevol/A3"

[volume]
value = 1298.6283305884593
exact = "12·12·10 − (π/2)·ρ̄·A = 1440 − (π/2)·6·15 = 1440 − 45π (Pappus: the triangle's area 15, its centroid 6 from the axis)"
tex = '12\cdot 12\cdot 10 - \tfrac{\pi}{2}\cdot 6\cdot 15 = 1440 - 45\pi'

[area]
value = 791.3621860530726
exact = "768 − 18π [the top's quarter-ring sector, (π/4)(9² − 3²)] − 15 − 15 [the triangles opened in the front and left faces] + 6π√34 [two cones: (π/2)(4.5 + 7.5)√34]"
tex = '768 - 18\pi - 15 - 15 + 6\pi\sqrt{34}'
+++

A square block with a V-shaped groove turned through its corner, a quarter of a ring about the
block's vertical edge, from the block's front face to its left face.

- The block: 0 ≤ x ≤ 12, 0 ≤ y ≤ 12, 0 ≤ z ≤ 10. Its vertical edge along the z axis is the axis
  the groove is turned about.
- The sketch: the triangle in the plane y = 0, on the block's front face, with corners (3, 0, 10),
  (9, 0, 10) and (6, 0, 5): a V with its flat edge level with the block's top.
- Turn the triangle about the z axis from the front face, towards +y, until it reaches the plane
  x = 0, the block's left face: a quarter turn, 90°. Cut away what it sweeps.

The groove opens through the block's top, and through the front face and the left face at its two
ends, a triangle in each. Its walls are two cones, the V's slopes, and nothing else.

**As a set:** with $\rho = \sqrt{x^2 + y^2}$,

$$\bigl([0,12]^2 \times [0,10]\bigr) \setminus \{\,5 + \tfrac{5}{3}|\rho - 6| \le z \le 10\,\}$$
