+++
title = "Five counterbored holes on a bolt circle round a bored disk"
tags = ["hole", "counterbore", "pattern", "cut", "planar", "cylinder", "analytic"]

[volume]
value = 50860.500106026599
exact = "π·40²·12 − π·12²·12 − 5(π·3.3²·12 + π(5.5² − 3.3²)·6.5) = 19200π − 1728π − 1282.6π = 16189.4π"
tex = '\pi\cdot 40^2\cdot 12 - \pi\cdot 12^2\cdot 12 - 5\left(\pi\cdot 3.3^2\cdot 12 + \pi\left(5.5^2 - 3.3^2\right)\cdot 6.5\right) = 16189.4\pi'

[area]
value = 14420.22443924251
exact = "2π·40² + 2π·40·12 − 2π·12² + 2π·12·12 + 5(−π·5.5² − π·3.3² + π(5.5² − 3.3²) + 2π·5.5·6.5 + 2π·3.3·5.5) = 4160π + 5·86.02π = 4590.1π"
tex = '2\pi\cdot 40^2 + 2\pi\cdot 40\cdot 12 - 2\pi\cdot 12^2 + 2\pi\cdot 12\cdot 12 + 5\left(-\pi\cdot 5.5^2 - \pi\cdot 3.3^2 + \pi\left(5.5^2 - 3.3^2\right) + 2\pi\cdot 5.5\cdot 6.5 + 2\pi\cdot 3.3\cdot 5.5\right) = 4590.1\pi'
+++

A round disk of radius 40 on the z axis, from z = 0 to z = 12, with a bore of radius 12 on the z
axis right through it, and five counterbored holes on a bolt circle of radius 28 about the z axis.

Each counterbored hole is drilled down from the top face (z = 12), along a vertical axis:

- the hole, radius 3.3, right through the disk to the bottom face (z = 0);
- the counterbore, radius 5.5 and 6.5 deep, from the top face down to a flat floor at z = 5.5.

The first hole's axis is the vertical line through (28, 0); each of the others is the first turned
about the z axis by 72°, 144°, 216° and 288°. The holes stay clear of the bore (the counterbores'
nearest point is 22.5 from the z axis, the bore 12), of the rim (furthest point 33.5, the rim 40)
and of each other. If you cut the holes with cylinders, make them run past the bottom face.

**As a set:** with $d_k$ the distance from $(x, y)$ to $(28\cos 72^\circ k,\ 28\sin 72^\circ k)$,

$$\{\,12 \le \rho \le 40,\ 0 \le z \le 12\,\} \setminus \bigcup_{k=0}^{4} \bigl(\{\,d_k \le 3.3\,\} \cup \{\,d_k \le 5.5,\ z \ge 5.5\,\}\bigr)$$
