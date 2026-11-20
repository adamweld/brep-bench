+++
title = "A cone frustum with a conical void inside it"
tags = ["boolean", "cut", "analytic", "cone", "void"]
origin = "OCCT tests/boolean/bopcut_simple/ZM5"

[volume]
value = 652.4040743954804
exact = "πh(A² + AB + B²)/3 with h = 10, A = 6, B = 3, less the same with h = 4, a = 1/2, b = 1"
tex = '\tfrac{\pi h}{3}\left(A^2 + AB + B^2\right) - \tfrac{\pi h_2}{3}\left(a^2 + ab + b^2\right),\quad h = 10,\ A = 6,\ B = 3,\ h_2 = 4,\ a = \tfrac12,\ b = 1'

[area]
value = 459.4876195801513
exact = "the outer frustum's whole surface π(A + B)√(h² + (A − B)²) + πA² + πB², plus the void's wall π(a + b)√(h₂² + (b − a)²) + πa² + πb²"
tex = '\pi(A + B)\sqrt{h^2 + (A - B)^2} + \pi A^2 + \pi B^2 + \pi(a + b)\sqrt{h_2^2 + (b - a)^2} + \pi a^2 + \pi b^2'
+++

A cone frustum standing on the xy plane, its axis the z axis, from z = 0 to z = 10: radius 6 at its base, radius 3 at its top. With a smaller frustum cut out of its inside: on the same axis, from z = 3 to z = 7, radius 1/2 at its lower end and radius 1 at its upper end. The smaller frustum touches nothing of the wider, so the result is a solid with a closed conical void, a second boundary shell inside the first.

**As a set:**

$$\{\rho \le 6 - \tfrac{3}{10}z,\ 0 \le z \le 10\} \setminus \{\rho \le \tfrac12 + \tfrac{1}{8}(z - 3),\ 3 \le z \le 7\}$$
