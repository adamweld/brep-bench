+++
title = "A prism with a Bezier wall grown by 1"
tags = ["offset", "grow", "freeform", "round", "extrusion"]
origin = "OCCT tests/offset/shape_type_i/G2"
settled = false
+++

The straight extrusion, 5 along +z from the plane z = 0, of the region of the xy plane bounded by
the line from (0, 0) to (10, 0), the cubic Bezier curve (a cubic B-spline of one span) with control
points (10, 0), (10, 5), (6, 8) and (0, 8), and the line from (0, 8) back to (0, 0). The region is
convex. So the solid has a curved side wall, the extrusion of the Bezier curve, and flat faces: the
bottom, the top and two walls, in the planes y = 0 and x = 0.

Grow it by 1: the solid of every point within 1 of it. The flat faces move out 1, the Bezier wall
moves out 1 along its normals, and the convex edges and corners are rounded, by quarters of
cylinders (and, along the curve, a quarter of a surface of tube radius 1 about the curve) and
eighths of balls of radius 1. The offset of a Bezier curve has no closed form, so the case is
unsettled until two kernels agree.

**As a set:**

$$\bigl(\text{the region} \times [0, 5]\bigr) \oplus B_1$$
