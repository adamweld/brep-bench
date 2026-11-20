+++
title = "Two equal cylinders crossing at right angles, their junction rounded"
tags = ["fillet", "concave", "cylinder", "saddle", "tangent"]
origin = "OCCT tests/blend/bfuseblend/B6, OCCT tests/blend/bfuseblend/B7"
settled = false
+++

Two solid cylinders of radius 4 and length 24, crossing at their middles at right angles: the first about the x axis from x = −12 to x = 12, the second about the y axis from y = −12 to y = 12, united into one solid. The cylinders' sides meet along two ellipses, in the planes x = y and x = −y, which cross each other at the two points (0, 0, 4) and (0, 0, −4), where the two sides touch, with a common tangent plane (z = ±4), instead of meeting at an angle.

Round that junction to radius 1: it is the concave blend between the two sides, a ball of radius 1 rolling in the four arcs of the ellipses' meeting, touching both sides. It dies away to nothing at each of the two points (0, 0, ±4), where the angle between the sides becomes 180° and the ball has nothing to fill.

**As a set:** the union of the two cylinders closed by a ball of radius 1 (grown by 1, then shrunk by 1), $(A \oplus B_1) \ominus B_1$, which is the union with every place a ball of radius 1 cannot reach between the two sides filled.

The round's volume and area have no closed form, so the case is unsettled until two kernels agree.
