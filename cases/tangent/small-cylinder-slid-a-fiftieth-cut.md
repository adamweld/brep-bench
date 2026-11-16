+++
title = "A small cylinder less itself slid a fiftieth of its radius"
tags = ["tangent", "boolean", "cut", "coincident", "thin", "cylinder", "small"]

[volume]
value = 3.37494374915622e-11
exact = "h(πr² − L), r = 0.00075, h = 0.0015, d = 0.000015, L = 2r²·acos(d/2r) − (d/2)√(4r² − d²)"
tex = 'h\left(\pi r^2 - L\right),\quad L = 2r^2\arccos\tfrac{d}{2r} - \tfrac{d}{2}\sqrt{4r^2 - d^2},\ r = 0.00075,\ h = 0.0015,\ d = 0.000015'

[area]
value = 7.113582720565784e-06
exact = "2(πr² − L) + 2πrh"
tex = '2\left(\pi r^2 - L\right) + 2\pi r h'
+++

A small cylinder, of radius 0.00075 and height 0.0015 about the z axis from z = 0, and a copy of
it slid 0.000015 along x, a fiftieth of its radius. The two share their top and bottom planes, and
their sides cross in two vertical lines, in the plane x = 0.0000075.

Take the cylinder less the copy: a crescent prism, 0.000015 thick at its widest and running to
nothing at the two lines where the sides cross. Its top and bottom lie in the copy's, and a kernel
that takes the two sides for one, by a tolerance that does not shrink with the model, finds
nothing left.

**As a set:**

$$\{\,x^2 + y^2 \le r^2,\ (x - d)^2 + y^2 \ge r^2,\ 0 \le z \le h\,\}$$
