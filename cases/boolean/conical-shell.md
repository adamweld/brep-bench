+++
title = "A conical shell"
tags = ["boolean", "cut", "cone", "analytic"]
origin = "OCCT boolean/volumemaker/A2"

[volume]
value = 678.5840131753953
exact = "(πh/3)(R² + Rr + r²) taken for the outside less the inside, h = 8: 896π/3 − 248π/3 = 216π"
tex = '\tfrac{\pi h}{3}\left(R^2 + Rr + r^2\right)\Big|_{\text{outside}} - \Big|_{\text{inside}} = \tfrac{896\pi}{3} - \tfrac{248\pi}{3} = 216\pi'

[area]
value = 675.4326639287812
exact = "π(R + r)·√(8² + 4²) for the outside and for the inside, and an annulus at each end: 48√5π + 24√5π + π(8² − 5²) + π(4² − 1²) = 72√5π + 54π"
tex = '48\sqrt5\,\pi + 24\sqrt5\,\pi + \pi\left(8^2 - 5^2\right) + \pi\left(4^2 - 1^2\right) = 72\sqrt5\,\pi + 54\pi'
+++

A hollow cone frustum on the z axis, open at both ends, whose wall is 3 wide measured across, along the radius. Its outside is the cone ρ = 8 − z/2, from radius 8 at z = 0 to radius 4 at z = 8, and its inside is the cone ρ = 5 − z/2, from radius 5 at z = 0 to radius 1 at z = 8. The two cones are parallel, with their apexes on the axis at z = 16 and z = 10. The ends are the flat rings in z = 0 and z = 8.

If you cut the inside with a cone, make it run past both ends.

**As a set:**

$$\{\,5 - \tfrac{z}{2} \le \rho \le 8 - \tfrac{z}{2},\ 0 \le z \le 8\,\}$$
