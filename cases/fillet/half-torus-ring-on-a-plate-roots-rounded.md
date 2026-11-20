+++
title = "A half-torus ring standing on a plate, its two roots rounded"
tags = ["fillet", "concave", "torus", "analytic"]
origin = "OCCT tests/blend/bfuseblend/A4, OCCT tests/blend/bfuseblend/A5, OCCT tests/blend/bfuseblend/A6"

[volume]
value = 12962.66385240327
exact = "12500 + π²·10·2² (the plate and the ring, half of a torus) + 2π·(M_outer + M_inner), M_outer + M_inner = 120 − 90·atan(4/3) − 40·atan(3/4) (Pappus, the two roots' sections): 12500 + 40π² + 240π − 180π·atan(4/3) − 80π·atan(3/4)"
tex = '12500 + 40\pi^2 + 240\pi - 180\pi\arctan\tfrac43 - 80\pi\arctan\tfrac34'

[area]
value = 6079.981944342195
exact = "3500 (the plate's bottom and sides) + (2500 − 160π) (its top outside the ring and its rounds' feet, the annulus from radius 6 to 14 being taken) + 40π² (the ring's half torus, 2π²·10·2) less the two bands of it under the rounds, plus the two rounds, together 60π² − 200π·atan(3/4) (tori patches, 2π∫ρ ds): 6000 − 160π − 200π·atan(3/4) + 100π²"
tex = '6000 - 160\pi - 200\pi\arctan\tfrac34 + 100\pi^2'
+++

A square plate from (−25, −25, −5) to (25, 25, 0), and standing on its top face a ring that is half a torus: the part above z = 0 of the torus about the z axis, whose tube has radius 2 and whose centre circle has radius 10 at z = 0. The ring's section through the axis is a half-disk of radius 2 about (ρ, z) = (10, 0), so its sides are vertical where it meets the plate, at the circles of radius 8 and 12. The two are one solid.

Round both of those circles, each to radius 3. A round is concave, a ball of radius 3 rolling round the root touching the plate and the ring. The outer round meets the plate on the circle of radius 14 and has its ball's centre at (ρ, z) = (14, 3); the inner round meets the plate on the circle of radius 6 and has its ball's centre at (6, 3). Each meets the ring where the line from the tube's centre (10, 0) to the ball's centre meets the tube, at height 6/5, radius 10 ± 8/5. The two rounds do not touch.

**As a set:** in the plane through the axis, with $u = |\rho - 10|$, the filled corners are the points with $z \ge 0$, $u \le 4$, $z \le \tfrac34 u$, outside the tube $u^2 + z^2 > 2^2$ and outside the disks of radius 3 about $(10 \pm 4, 3)$, turned about the axis.
