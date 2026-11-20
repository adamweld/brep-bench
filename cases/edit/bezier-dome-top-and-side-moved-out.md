+++
title = "The Bézier dome block, its top moved up and a side moved out"
tags = ["edit", "move-face", "nurbs", "bezier", "surface"]
origin = "OCCT tests/offset/faces_type_i/N1, N2"
settled = false
+++

The block under a bicubic Bézier dome of `nurbs/bezier-dome`: standing on the square
0 ≤ x ≤ 20, 0 ≤ y ≤ 20 of the xy plane, its floor flat at z = 0, its four sides upright in the planes
x = 0, x = 20, y = 0 and y = 20, and its top the bicubic Bézier patch S(u, v) of that case, x = 20u,
y = 20v.

Then edit the solid directly: move the top 1 along its normal, outward and so upward, and move
the side y = 20 out by 2, to y = 22. The new top is the offset surface of the patch, the points
S(u, v) + n(u, v) with n the unit normal pointing up, taken from the cubic polynomial S continued past
the square wherever it is needed: the offset surface is shifted sideways by the horizontal part of
n, so it does not stand over exactly the square, and near each side it must be continued past
the old edge, or trimmed, to reach the side's plane. The sides x = 0, x = 20 and y = 0 and the
floor are as before, the side y = 20 is now y = 22, and the new top runs over the strip between 20
and 22 as the same continued offset surface.

So the result is the solid with 0 ≤ x ≤ 20, 0 ≤ y ≤ 22 between z = 0 and that top. The area of
a patch's offset has no closed form, and the continuation past the edge makes the volume depend on
that one polynomial; the case is unsettled until two kernels agree.
