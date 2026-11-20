+++
title = "A quarter-turn groove cut round a post that stands out of a box"
tags = ["feature", "revolve", "cut", "cylinder", "analytic"]
origin = "OCCT tests/feat/featrevol/H9"

[volume]
value = 19633.62817986669
exact = "abc + πr²L − (1/4)π(r² − s²)w, a = b = 30, c = 20, r = 6, L = 15, s = 4, w = 4: = 18000 + 520π"
tex = 'abc + \pi r^2 L - \tfrac{1}{4}\pi\left(r^2 - s^2\right)w,\quad a = b = 30,\ c = 20,\ r = 6,\ L = 15,\ s = 4,\ w = 4:\ = 18000 + 520\pi'

[area]
value = 4800.336233567702
exact = "(2ab + 2(a + b)c − πr²) + 2πrL + πr² − (π/2)rw + (π/2)sw + 2·(π/4)(r² − s²) + 2(r − s)w: = 4216 + 186π"
tex = '\left(2ab + 2(a + b)c - \pi r^2\right) + 2\pi r L + \pi r^2 - \tfrac{\pi}{2} r w + \tfrac{\pi}{2} s w + 2\cdot\tfrac{\pi}{4}\left(r^2 - s^2\right) + 2(r - s)w:\ = 4216 + 186\pi'
+++

A box from the origin to (30, 30, 20), and on its side face x = 30 a post: a cylinder of radius 6
whose axis is the line y = 15, z = 10 along +x, from x = 30 to x = 45, united with the box.

A groove is cut round the post by revolving a square sketch about the post's axis through a quarter
turn. The square is 4 by 4 in the plane y = 15, with 34 ≤ x ≤ 38 and 4 ≤ z − 10 ≤ 8, so it lies 4 to 8
from the post's axis, on its +z side. It is turned about the axis (the line y = 15, z = 10 along +x)
through 90°, right-handed, so that the edge of the square nearest the axis, directly above the axis,
ends up at its −y side. Everything the square sweeps is cut away.

The groove is the part of the post with 34 ≤ x ≤ 38 and 4 ≤ r ≤ 8 from the axis, in the quarter
of the post from straight above the axis round to −y; the post's core of radius 4 stays whole, and
the box is untouched. The groove opens out through the post's wall and has two flat ends, one at each
end of the quarter.

**As a set:**

With u = y − 15, v = z − 10 and r = √(u² + v²):

$$\Bigl(\bigl([0,30]\times[0,30]\times[0,20]\bigr) \cup \{\,r \le 6,\ 30 \le x \le 45\,\}\Bigr) \setminus \{\,34 \le x \le 38,\ 4 \le r \le 8,\ v \ge 0,\ u \le 0\,\}$$
