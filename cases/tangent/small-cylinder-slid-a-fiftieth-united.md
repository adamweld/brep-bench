+++
title = "A small cylinder united with itself slid a fiftieth of its radius"
tags = ["tangent", "boolean", "union", "coincident", "cylinder", "small"]

[volume]
value = 2.68446823895795e-09
exact = "h(2πr² − L), r = 0.00075, h = 0.0015, d = 0.000015, L = 2r²·acos(d/2r) − (d/2)√(4r² − d²)"
tex = 'h\left(2\pi r^2 - L\right),\quad L = 2r^2\arccos\tfrac{d}{2r} - \tfrac{d}{2}\sqrt{4r^2 - d^2},\ r = 0.00075,\ h = 0.0015,\ d = 0.000015'

[area]
value = 1.0692875205888053e-05
exact = "2(2πr² − L) + 2h(2πr − 2r·acos(d/2r))"
tex = '2\left(2\pi r^2 - L\right) + 2h\left(2\pi r - 2r\arccos\tfrac{d}{2r}\right)'
+++

A small cylinder, of radius 0.00075 and height 0.0015 about the z axis from z = 0, and a copy of
it slid 0.000015 along x, a fiftieth of its radius. The two share their top and bottom planes, and
their sides cross in two vertical lines, in the plane x = 0.0000075.

Take their union: the prism over the two circles' union, its sides meeting in the two lines where
they cross.

**As a set:**

$$\{\,x^2 + y^2 \le r^2\ \text{or}\ (x - d)^2 + y^2 \le r^2,\ 0 \le z \le h\,\}$$
