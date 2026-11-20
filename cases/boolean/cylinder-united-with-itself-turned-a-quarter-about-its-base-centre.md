+++
title = "A cylinder united with itself turned a quarter turn about its base's centre"
tags = ["boolean", "union", "cylinder", "crossing", "analytic"]
origin = "OCCT tests/bugs/modalg_6/bug27274"

[volume]
value = 114.99703947692505
exact = "2·π·r²·h − (quarter of the bicylinder) = 2π·4·5 − 4r³/3 = 40π − 32/3, r=2, h=5"
tex = '2\pi r^2 h - \tfrac{4}{3}r^3 = 40\pi - \tfrac{32}{3}'

[area]
value = 147.36281798666926
exact = "2(2πrh + 2πr²) − 2·(2r²) − 2·(πr²/2) = 4πrh + 3πr² − 4r² = 52π − 16"
tex = '4\pi r h + 3\pi r^2 - 4r^2 = 52\pi - 16'
+++

A cylinder of radius 2 and height 5 with its axis along the z axis, its base a disc in the xy plane centred on the origin and its top at z = 5. Make a second cylinder by turning the first a quarter turn about the y axis through the origin, so that its axis lies along the +x axis, its base is the disc in the yz plane centred on the origin, and its far end is at x = 5. The two share the centre of their bases and overlap near it, then diverge. Unite them.

**As a set:**

$$\{\,x^2+y^2 \le 4,\ 0 \le z \le 5\,\} \cup \{\,y^2+z^2 \le 4,\ 0 \le x \le 5\,\}$$
