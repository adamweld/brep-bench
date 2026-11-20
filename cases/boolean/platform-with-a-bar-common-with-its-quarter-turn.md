+++
title = "A drilled platform with a bar, common with itself turned a quarter turn"
tags = ["boolean", "common", "analytic", "box", "cylinder", "hole"]
origin = "OCCT heal/bug26244"

[volume]
value = 22121.62842102152
exact = "60·60·6 − 4·π·4²·6 + 12·12·12"
tex = '60\cdot60\cdot6 - 4\pi\cdot4^2\cdot6 + 12\cdot12\cdot12'

[area]
value = 9417.061929829746
exact = "(3600 − 64π) + (3600 − 64π − 144) + 4·60·6 + 4·2π·4·6 + 4·12·12 + 12²"
tex = '(3600-64\pi)+(3600-64\pi-144)+1440+192\pi+576+144'
+++

Make a platform P: the plate from (−30, −30, 0) to (30, 30, 6), with four through holes of radius 4 about the vertical lines through (x, y) = (±22, ±22), and standing on its top face a bar from (−20, −6, 6) to (20, 6, 18), long in x. Make a copy of P turned a quarter turn (90°, anticlockwise seen from above) about the z axis through the origin; in the copy the bar is long in y. The solid is the part common to P and its turned copy: the plate with its four holes (the holes map onto one another) and, on it, the square block from (−6, −6, 6) to (6, 6, 18) where the two bars cross.

**As a set:**

$$P\cap R_{90^\circ}P$$
