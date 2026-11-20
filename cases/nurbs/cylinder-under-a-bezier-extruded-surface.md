+++
title = "A cylinder cut away above an extruded Bézier curve"
tags = ["nurbs", "bezier", "cut", "cylinder", "extrusion"]
origin = "OCCT mesh/bug30008_2"
settled = false
+++

A cylinder of radius 10 about the z axis, from z = 0 to z = 20. In the xz plane take the cubic Bézier curve with control points (−12, 6), (−4, 14), (4, 2), (12, 10) (x, z); it runs once from x = −12 to x = 12 with x rising evenly along it, so it is a graph z = g(x). Sweep it straight along y, from y = −15 to y = 15, into a freeform surface. Keep of the cylinder only what lies on or below that surface, that is the points with z ≤ g(x): the cylinder's top is replaced by a wavy freeform roof, and the cylinder's side now ends in a curve running round under it.

**As a set:**

$$\{\rho\le10,\ 0\le z\le20,\ z\le g(x)\}$$
