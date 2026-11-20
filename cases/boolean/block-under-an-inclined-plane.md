+++
title = "A block under an inclined plane"
tags = ["boolean", "cut", "analytic", "planar"]
origin = "OCCT boolean/volumemaker/A3, OCCT boolean/volumemaker/A7, OCCT boolean/volumemaker/C2"

[volume]
value = 210.0
exact = "6·∫₀¹⁰(2 + 0.3x)dx = 6·35"
tex = '6\int_0^{10}(2 + 0.3x)\,dx = 6\cdot 35'

[area]
value = 234.6418390534633
exact = "60 + 6·2 + 6·5 + 2·35 + 60√1.09"
tex = '60 + 6\cdot 2 + 6\cdot 5 + 2\cdot 35 + 60\sqrt{1.09}'
+++

The solid bounded by six planes: z = 0 below, y = 0 and y = 6 at the sides, x = 0 and x = 10 at the ends, and above, the plane z = 2 + 0.3x, which is 2 high at x = 0 and 5 high at x = 10. Five of its faces are axis-aligned and the top is inclined.

**As a set:**

$$\{\,0 \le x \le 10,\ 0 \le y \le 6,\ 0 \le z \le 2 + 0.3x\,\}$$
