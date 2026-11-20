+++
title = "A stepped block with a V-groove turned across its step"
tags = ["feature", "groove", "revolve", "up-to-face", "cone", "planar"]
settled = false
origin = "OCCT feat/featrevol/D1, OCCT feat/featrevol/D2, OCCT feat/featrevol/D3, OCCT feat/featrevol/D4"
+++

A stepped block, two boxes joined, with a V-shaped groove turned round the block's vertical edge
so that it crosses the step: its upper part runs out into the air beside the upper box and in
through the upper box's two sides, and its lower part cuts into the lower box's top.

- The lower box: 0 ≤ x ≤ 20, 0 ≤ y ≤ 20, 0 ≤ z ≤ 6.
- The upper box: 0 ≤ x ≤ 10, 0 ≤ y ≤ 10, 6 ≤ z ≤ 12, standing on the lower box's corner at the
  origin. The two are one solid; its vertical edge along the z axis runs from z = 0 to z = 12.
- The sketch: the triangle in the plane y = 0 with corners (6, 0, 12), (14, 0, 12) and (10, 0, 2),
  a V with its flat edge level with the upper box's top.
- Turn it about the z axis, from the plane y = 0 towards +y, until it reaches the plane x = 0: a
  quarter turn, 90°. Cut away what it sweeps.

**As a set:** with $\rho = \sqrt{x^2 + y^2}$, $\varphi$ the angle about the z axis from +x, and K the
stepped block,

$$K \setminus \{\,2 + 2.5\,|\rho - 10| \le z \le 12,\ 0 \le \varphi \le 90^\circ\,\}$$

The groove's walls are cones about the z axis, which meet the upper box's side faces (the planes
x = 10 and y = 10) in curves, so the solid's volume and area have no short closed form and are not
given here.
