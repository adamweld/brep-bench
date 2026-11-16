+++
title = "A small cylinder less itself raised a hundredth of its height"
tags = ["tangent", "boolean", "cut", "cylinder", "coincident", "small", "thin"]

[volume]
value = 2.650718801466388e-11
exact = "πr²t, r = 0.00075, h = 0.0015, t = 0.000015"
tex = '\pi r^2 t,\quad r = 0.00075,\ h = 0.0015,\ t = 0.000015'

[area]
value = 3.6049775699942876e-06
exact = "2πr² + 2πrt"
tex = '2\pi r^2 + 2\pi r t'
+++

A small cylinder, of radius 0.00075 and height 0.0015 about the z axis from z = 0, and a copy of
it raised by 0.000015 along z, a hundredth of its height. The copy's side lies in the cylinder's
own side.

Take the cylinder less the copy: the disc under the copy, from z = 0 to z = 0.000015, a
hundredth as thick as it is wide. Its side lies in the copy's side and its top is the copy's
bottom. A kernel that takes the copy's bottom for the cylinder's own, by a tolerance that does
not shrink with the model, finds nothing left.

**As a set:**

$$\{\,x^2 + y^2 \le 0.00075^2,\ 0 \le z \le 0.000015\,\}$$
