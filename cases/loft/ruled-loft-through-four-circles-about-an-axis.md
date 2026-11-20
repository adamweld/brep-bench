+++
title = "A ruled loft through four circles stepped round a distant axis"
tags = ["loft", "ruled", "circle"]
origin = "OCCT mesh/bug32241"
settled = false
+++

Four circles of radius 2, numbered k = 0, 1, 2, 3, stepped round the z axis by 30° each. With a = 30°·k, circle k has its center at (10 cos a, 10 sin a, 0) and lies in the vertical plane through the z axis and its center: it is the set of points (10 cos a, 10 sin a, 0) + 2 (cos φ)(cos a, sin a, 0) + 2 (sin φ)(0, 0, 1), for φ from 0 to 2π. Join the circles by a ruled loft: between circle k and circle k + 1 the surface is the union of the straight segments from the point of circle k at φ to the point of circle k + 1 at the same φ, for every φ. Close the two ends with the flat discs of circles 0 and 3. The solid is the region bounded by the three ruled bands and the two discs; it is a chain of three slightly bent frusta-like pieces, a coarse stand-in for a quarter of a torus ring.
