+++
title = "A knife edge of 5° rounded"
tags = ["fillet", "prism", "planar", "cylinder", "sharp-edge", "small"]
origin = "brep-conformance 1.0.0, fillets/knife-edge"

[volume]
value = 24.964002910601035
exact = "6(50 tan β − r²(cot(β/2) − (π − β)/2)), β = 5°, r = 0.1"
tex = '6\left(50\tan\beta - r^2\left(\cot\tfrac{\beta}{2} - \tfrac{\pi - \beta}{2}\right)\right),\quad \beta = 5^\circ,\ r = 0.1'

[area]
value = 108.14792142416658
exact = "2P + 6(10 + 10 tan β + 10/cos β − 2r cot(β/2) + r(π − β)), P = 50 tan β − r²(cot(β/2) − (π − β)/2), β = 5°, r = 0.1"
tex = '2P + 6\left(10 + 10\tan\beta + \tfrac{10}{\cos\beta} - 2r\cot\tfrac{\beta}{2} + r(\pi - \beta)\right),\quad P = 50\tan\beta - r^2\left(\cot\tfrac{\beta}{2} - \tfrac{\pi - \beta}{2}\right)'
+++

A wedge 6 high, from z = 0 to z = 6, on the triangle (0, 0), (10, 0), (10, 10 tan 5°): its edge
along the z axis is a knife edge where two faces meet at 5°. Round that edge with a fillet of
radius 0.1.

The round's ball touches each face 0.1 cot 2.5° ≈ 2.290 from the edge, far back along the blade
for so small a radius, and the round turns through 175°, its axis on the bisecting plane.

**As a set:**

$$\Bigl(\mathrm{poly}\bigl((0, 0), (10, 0), (10, 10\tan 5^\circ)\bigr) \setminus \bigl(\mathrm{poly}\bigl((0, 0), (t, 0), (t\cos 5^\circ, t\sin 5^\circ)\bigr) \setminus \{\, (x - t)^2 + (y - 0.1)^2 \le 0.1^2 \,\}\bigr)\Bigr) \times [0, 6],\quad t = 0.1\cot 2.5^\circ$$
