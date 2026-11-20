+++
title = "A cylinder and a longer, narrower coaxial cylinder, their common part"
tags = ["boolean", "common", "analytic", "cylinder"]
origin = "OCCT tests/boolean/bopcommon_simple/ZD3"

[volume]
value = 75.39822368615503
exact = "π·2²·6"
tex = '\pi \cdot 2^2 \cdot 6'

[area]
value = 100.53096491487338
exact = "2π·2·6 + 2·π·2²"
tex = '2\pi \cdot 2 \cdot 6 + 2\pi \cdot 2^2'
+++

A cylinder of radius 4 standing on the xy plane, its axis the z axis, from z = 0 to z = 6. A second cylinder on the same axis, of radius 2, from z = −3 to z = 6: it is longer, its top disc level with the first's top disc and its other end beyond the first's base. Take the part common to the two.

The narrower cylinder lies inside the wider one and touches its wall nowhere; the common part is the narrower one's length from z = 0 to z = 6, cut off flat by the first's base plane, with both of its ends flat.

**As a set:**

$$\{\,\rho \le 4,\ 0 \le z \le 6\,\} \cap \{\,\rho \le 2,\ -3 \le z \le 6\,\}$$
