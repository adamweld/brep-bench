+++
title = "An elliptic cylinder shelled with its top open: an elliptic cup"
tags = ["shell", "inward", "ellipse", "analytic"]
origin = "OCCT tests/bugs/modalg_2/bug5805_10, OCCT tests/bugs/modalg_2/bug5805_11"

[volume]
value = 248.5925456921219
exact = "πab·h − (πab − pt + πt²)(h − t), p = 4aE(1 − b²/a²), a = 5, b = 3, h = 10, t = 1: the cylinder less its cavity"
tex = '\pi a b h - \left(\pi a b - p t + \pi t^2\right)(h - t),\quad p = 4aE\left(1 - \tfrac{b^2}{a^2}\right),\ a = 5,\ b = 3,\ h = 10,\ t = 1'

[area]
value = 522.712090247642
exact = "p·h + 2πab + (p − 2πt)(h − t): the outside, side p·h and bottom πab; the rim, the ellipse less the inner curve's inside; the cavity's side (p − 2πt)(h − t) and floor, whose area with the rim's makes πab again"
tex = 'p h + 2\pi a b + (p - 2\pi t)(h - t)'
+++

The elliptic cylinder of `geometry/elliptic-cylinder`: the ellipse x²/25 + y²/9 ≤ 1 in the plane
z = 0 extruded straight up to z = 10.

Shell it inward to a wall 1 thick, open at its top face, z = 10. The outside of the wall is its
side and its bottom, unmoved, and the inside is each of them moved 1 inward along its normal, onto
the surface offset 1 from it. The moved faces are extended or trimmed where they meet one another,
and run on until they meet the surface of the opening, which is left as a rim between the outside
and the inside.

The side moved in is the extrusion of the curve at distance 1 inside the ellipse, which is no
ellipse; the ellipse curves most tightly at the ends of its long axis, with radius of curvature
b²/a = 1.8, more than 1, so that curve is smooth. The bottom moved in is the plane z = 1. So the
result is the cylinder less the solid over that inner curve from z = 1 to z = 10, a cup, its rim
the band between the ellipse and the inner curve at z = 10. The area inside the inner curve and
its length follow from the ellipse's own (Steiner): πab − pt + πt² and p − 2πt, p the ellipse's
perimeter, E the complete elliptic integral of the second kind.

**As a set:**

$$\{\,\tfrac{x^2}{25} + \tfrac{y^2}{9} \le 1,\ 0 \le z \le 10\,\} \setminus \bigl(\{\,\tfrac{x^2}{25} + \tfrac{y^2}{9} \le 1,\ z \ge 1\,\} \ominus B_{1}\bigr)$$
