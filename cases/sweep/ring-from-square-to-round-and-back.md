+++
title = "A closed ring whose section turns from a square to a circle and back"
tags = ["sweep", "closed-path", "variable-section", "loft"]
origin = "OCCT tests/bugs/modalg_6/bug28468_2"
settled = false
+++

A ring about the z axis whose section is a square at one place and a circle at the opposite place. The centerline is the circle of radius 20 about the z axis in the xy plane, parameterised by the angle θ from +x (θ from 0 to 360°, all the way round, with no end faces).

At every angle θ the section lies in the half-plane through the z axis at that angle, centered on the centerline. Describe a point of it by its offset u outward from the centerline (along the horizontal direction away from the z axis) and its height z. The section is the star-shaped region about its center whose boundary, at direction α measured in the (u, z) plane from the outward direction, lies at the distance

r(α, t) = (1 − t)·3 / max(|cos α|, |sin α|) + t·3

from the center. At t = 0 this is the square of half-width 3 with its sides along u and z; at t = 1 it is the circle of radius 3. The blend parameter follows the angle by t = θ/180° for θ from 0 to 180° and t = 2 − θ/180° for θ from 180° to 360°: the section is the square at θ = 0, the circle at θ = 180°, and back to the square at θ = 360°, the two halves mirror images of each other. Every section is therefore a radial blend, point for point at equal direction α, between the square and the circle.

The result is one closed solid. It has no closed form.

**As a set:**

$$\{\,(\rho, \varphi, z) : r\bigl(\alpha, t(\varphi)\bigr) \ge \sqrt{(\rho - 20)^2 + z^2},\ \tan\alpha = z/(\rho - 20)\,\}$$
