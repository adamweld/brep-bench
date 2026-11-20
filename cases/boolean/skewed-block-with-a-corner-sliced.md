+++
title = "A skewed block with a corner sliced off"
tags = ["boolean", "cut", "analytic", "planar", "oblique"]
origin = "OCCT boolean/volumemaker/B3, OCCT boolean/volumemaker/B4, OCCT boolean/volumemaker/B6, OCCT boolean/volumemaker/C1"

[volume]
value = 208.24615384615385
exact = "(6³/det M)(1 − 1/48), det M = 65/64: the skewed block less the corner tetrahedron cut at half its three edges (t³/6 of it, t = ½)"
tex = '\frac{6^3}{\det M}\left(1 - \tfrac{1}{48}\right),\quad \det M = \tfrac{65}{64}'

[area]
value = 215.11385411886178
exact = "sum of the seven faces' areas, each a planar polygon whose corners are the intersections of three of the planes (done in 40-digit arithmetic)"
tex = '\sum_{i=1}^{7} A_i'
+++

A block whose six faces are oblique to every axis, with one corner sliced off by a seventh plane. With $u = x + y/4$, $v = y + z/4$ and $w = z + x/4$, the block is the set where u, v and w each lie between 0 and 6: three pairs of parallel planes, a skewed block (a parallelepiped) with a corner at the origin. Its far corner, where u = v = w = 6, lies at (4.8, 4.8, 4.8); the plane x + y + z = 12 slices it off, passing through the midpoints of the three edges that meet there.

**As a set:**

$$\{\,0 \le x + \tfrac{y}{4} \le 6,\ 0 \le y + \tfrac{z}{4} \le 6,\ 0 \le z + \tfrac{x}{4} \le 6,\ x + y + z \le 12\,\}$$
