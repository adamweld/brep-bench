+++
title = "A cylinder sector from a rectangle turned 120° about one side"
tags = ["primitive", "revolve", "sector", "analytic"]
origin = "FreeCAD tests/src/Mod/Part/App/FeatureRevolution.cpp::testAngle"

[volume]
value = 104.71975511965978
exact = "(θ/2)r²h, θ = 2π/3, r = 5, h = 4: = 100π/3"
tex = '\tfrac{\theta}{2}\,r^2 h = \tfrac{100\pi}{3},\quad \theta = \tfrac{2\pi}{3},\ r = 5,\ h = 4'

[area]
value = 134.2477796076938
exact = "θr² + θrh + 2rh: floor and top (θr²/2 each), the curved wall, and two flat ends of r·h each: = 30π + 40"
tex = '\theta r^2 + \theta r h + 2 r h = 30\pi + 40'
+++

In the xz plane, take the rectangle with x from 0 to 5 and z from 0 to 4; one side of it, the one at
x = 0, lies on the z axis. Revolve it 120° about the z axis, right-handed: from +x toward +y.

The solid is a sector of a cylinder of radius 5 and height 4, with its point on the z axis. Its faces
are the floor and the top (each a sector of a disk of radius 5 and angle 120°), the curved wall, and
two flat rectangular ends, 5 by 4, one in the plane y = 0 and one in the plane through the z axis at
120° from it.

**As a set:**

$$\{\,0 \le \rho \le 5,\ 0 \le \varphi \le 120^\circ,\ 0 \le z \le 4\,\}$$
