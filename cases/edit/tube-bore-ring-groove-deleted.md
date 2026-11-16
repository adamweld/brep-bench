+++
title = "A tube's internal ring groove deleted: the bore is whole again"
tags = ["edit", "delete-face", "cylinder", "planar", "groove", "hole", "analytic"]

[volume]
value = 5277.875658030853
exact = "π(10² − 4²)·20 = 1680π"
tex = '\pi\left(10^2 - 4^2\right)\cdot 20 = 1680\pi'

[area]
value = 2287.0794518133694
exact = "2π(10² − 4²) + 2π·10·20 + 2π·4·20 = 728π"
tex = '2\pi\left(10^2 - 4^2\right) + 2\pi\cdot 10\cdot 20 + 2\pi\cdot 4\cdot 20 = 728\pi'
+++

A tube about the z axis from z = 0 to z = 20, outer radius 10, its bore of radius 4, with a ring
groove turned into the bore: everything inside radius 6 from z = 8 to z = 12 cut away. The groove
has three faces, a floor (the cylinder of radius 6) and two walls (the flat rings from radius 4 to 6
at z = 8 and z = 12).

Then edit the tube directly: delete the groove's floor and both its walls together. Deleting faces takes them out of the solid and closes the gap they leave with the faces round them,
each run on along its own surface until they meet.
The faces round the groove are the bore below it and above it, which are one surface, so the bore
runs on across the groove and the two become one face again.

The result is the plain tube, with four faces.

**As a set:**

$$\{\,16 \le x^2 + y^2 \le 100,\ 0 \le z \le 20\,\}$$
