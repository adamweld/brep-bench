+++
title = "A cylinder and a narrower cylinder crossing beside its axis and touching its wall at a point, their common part"
tags = ["tangent", "boolean", "common", "analytic", "cylinder"]
origin = "OCCT tests/boolean/bopcommon_simple/ZE7"

[volume]
value = 81.32252696456702
exact = "∫₀⁴ 2√(16 − y²) · 2√(4 − (y − 2)²) dy: the thin disc (y − 2)² + z² ≤ 4 swept along x between the wide cylinder's walls x = ±√(16 − y²)"
tex = '\int_0^4 2\sqrt{16-y^2}\;2\sqrt{4-(y-2)^2}\,dy'

[area]
value = 107.56054720319175
exact = "∫₀^{2π} 4√(16 − (2 + 2cos t)²) dt for the narrow wall, plus ∫₀^π 8√(4 − (4 sin φ − 2)²) dφ for the wide wall"
tex = '\int_0^{2\pi} 4\sqrt{16-(2+2\cos t)^2}\,dt + \int_0^{\pi} 8\sqrt{4-(4\sin\varphi-2)^2}\,d\varphi'
+++

A cylinder of radius 4 and length 12 along the z axis, from z = −6 to z = 6. A second cylinder of radius 2 and the same length along the x axis, from x = −6 to x = 6, but with its axis moved sideways to the line y = 2, z = 0. Take the part common to the two.

The narrow cylinder's extreme line y = 4 (z = 0) lies exactly on the wide cylinder's wall's tangent plane y = 4: the two walls touch at the single point (0, 4, 0), where they are tangent. Elsewhere the narrow cylinder lies inside the wide one for y < 4, and it leaves through the wide one's wall on both sides in x.

**As a set:**

$$\{\,\rho \le 4,\ |z| \le 6\,\} \cap \{\,(y-2)^2 + z^2 \le 2^2,\ |x| \le 6\,\}$$
