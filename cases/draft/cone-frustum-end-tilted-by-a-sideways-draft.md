+++
title = "A cone's end face drafted about a diameter, the pull across the axis"
tags = ["draft", "planar", "cone", "analytic"]
origin = "OCCT tests/draft/angle/A9"

[volume]
value = 3516.2078765888994
exact = "(π/3)R²Z − P(Z − H)/3, the cone to its apex less the cone above the slanted plane: R = 10, Z = R/tan 15°, H = 20, P = πc²/(1 − e²)^{3/2}, c = (Z − H)·tan 15°, e = tan 10°·tan 15°"
tex = '\tfrac{\pi}{3}R^2 Z - \tfrac{1}{3}P\,(Z - H),\quad R = 10,\ Z = \frac{R}{\tan 15^\circ},\ H = 20,\ P = \frac{\pi c^2}{(1 - e^2)^{3/2}},\ c = (Z - H)\tan 15^\circ,\ e = \tan 10^\circ \tan 15^\circ'

[area]
value = 1334.5963629756757
exact = "πR² + (πR² − P)/sin 15° + P·√(1 + tan²10°), with Z, P as for the volume"
tex = '\pi R^2 + \frac{\pi R^2 - P}{\sin 15^\circ} + P\sqrt{1 + \tan^2 10^\circ},\quad Z,\ P\ \text{as for the volume}'
+++

A frustum of a cone about the z axis: its floor the disk of radius 10 at z = 0, its side the cone
whose half-angle is 15°, ρ = 10 − z·tan 15°, up to z = 20, where its top face is the disk of radius
10 − 20·tan 15° ≈ 4.641.

Draft its top face by 10°, with the pull direction +x and the neutral plane x = 0: the top face lies
along the pull direction already, so it turns about the line where it crosses the neutral plane, the
diameter x = 0, z = 20 of its circle, until it leans 10° from the x direction. It rises toward +x,
becoming the plane z = 20 + x·tan 10°. The side and the floor are not drafted. The top is replaced
by the tilted plane: the cone is extended up to it on the +x half, where the plane is above z = 20,
and cut down to it on the −x half, where it is below; the plane crosses the cone in an ellipse.

**As a set:**

$$\{\,\rho \le 10 - z\tan 15^\circ,\ 0 \le z \le 20 + x\tan 10^\circ\,\}$$
