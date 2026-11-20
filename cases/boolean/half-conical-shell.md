+++
title = "Half a conical shell"
tags = ["boolean", "cut", "analytic", "cone", "planar"]
origin = "OCCT boolean/volumemaker/A2"

[volume]
value = 219.91148575128553
exact = "π/2·∫₀¹⁰((6 − 0.3z)² − (4 − 0.3z)²)dz = π/2·∫(20 − 1.2z)dz = 70π"
tex = '\tfrac{\pi}{2}\int_0^{10}\left[(6 - 0.3z)^2 - (4 - 0.3z)^2\right]dz = 70\pi'

[area]
value = 313.5766287575907
exact = "½·π(6² − 4²) + ½·π(3² − 1²) + 2·∫₀¹⁰(2)dz + ½π(6 + 3)√109 + ½π(4 + 1)√109"
tex = '\tfrac12\pi(6^2 - 4^2) + \tfrac12\pi(3^2 - 1^2) + 2\int_0^{10} 2\,dz + \tfrac12\pi(6 + 3)\sqrt{109} + \tfrac12\pi(4 + 1)\sqrt{109}'
+++

The part with x ≥ 0 of a conical shell round the z axis, between z = 0 and z = 10. The shell lies between two cones that share the axis and the slope: the outside cone has radius 6 at z = 0 and 3 at z = 10, the inside cone radius 4 at z = 0 and 1 at z = 10. So the shell is 2 thick measured square to the axis. Three planes bound it besides the cones: z = 0, z = 10, and x = 0, which cuts it in half.

**As a set:**

$$\{\,4 - 0.3z \le \rho \le 6 - 0.3z,\ 0 \le z \le 10,\ x \ge 0\,\}$$
