+++
title = "A square swept once round a circle"
tags = ["sweep", "planar-path", "analytic"]
origin = "OCCT tests/bugs/modalg_7/bug26568"

[volume]
value = 125.66370614359173
exact = "A·2π·ρ̄ = 4·2π·5 = 40π (Pappus: the square's area 4, its centroid 5 from the axis)"
tex = '4\cdot 2\pi\cdot 5 = 40\pi'

[area]
value = 251.32741228718345
exact = "2π·4·2 + 2π·6·2 + 2·π(6² − 4²) = 80π: the inner and outer cylinders and the two annuli"
tex = '2\pi\cdot 4\cdot 2 + 2\pi\cdot 6\cdot 2 + 2\pi\left(6^2 - 4^2\right) = 80\pi'
+++

The square with corners (4, 0, 0), (6, 0, 0), (6, 0, 2) and (4, 0, 2), in the plane y = 0, swept
once all the way round the circle of radius 4 about the z axis in the xy plane, which starts at
the square's corner (4, 0, 0) and runs counterclockwise. The square stays in the plane through the
z axis as it goes, and the sweep ends where it began, so the result is a closed ring of square
section.

**As a set:**

$$\{\,4 \le \rho \le 6,\ 0 \le z \le 2\,\}$$
