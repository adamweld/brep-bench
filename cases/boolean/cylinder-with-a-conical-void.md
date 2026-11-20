+++
title = "A cylinder with a conical void inside it"
tags = ["boolean", "cut", "analytic", "cone", "void"]
origin = "OCCT tests/boolean/bopcut_simple/ZJ8"

[volume]
value = 756.0766319639436
exact = "250π − πh(a² + ab + b²)/3 with h = 4, a = 1, b = 2: 250π − 28π/3"
tex = '250\pi - \tfrac{\pi h}{3}\left(a^2 + ab + b^2\right) = 250\pi - \tfrac{28\pi}{3}'

[area]
value = 525.8062163366635
exact = "outer 2π·5·10 + 2π·5², plus the void's wall π(a + b)·√(h² + (b − a)²) + πa² + πb², a = 1, b = 2, h = 4"
tex = '2\pi\cdot 5\cdot 10 + 2\pi\cdot 5^2 + \pi(a + b)\sqrt{h^2 + (b - a)^2} + \pi a^2 + \pi b^2'
+++

A cylinder of radius 5 and height 10 standing on the xy plane, its axis the z axis, with a cone frustum cut out of its inside: on the same axis, from z = 3 to z = 7, radius 1 at its lower end and radius 2 at its upper end. The frustum touches nothing of the cylinder, so the result is a solid with a closed conical void in it, a second boundary shell inside the first.

**As a set:**

$$\{\rho \le 5,\ 0 \le z \le 10\} \setminus \{\rho \le 1 + \tfrac{1}{4}(z - 3),\ 3 \le z \le 7\}$$
