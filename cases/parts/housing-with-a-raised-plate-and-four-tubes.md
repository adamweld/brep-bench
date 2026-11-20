+++
title = "A housing: rounded base with a hole, a raised plate, four bored tubes"
tags = ["boolean", "cut", "fuse", "analytic", "cylinder", "box", "hole"]
origin = "OCCT heal/bug26219_gehause_rohteil"

[volume]
value = 42202.12410081432
exact = "10(3744 + 64π − 144π) + 4(1800 − 144π) + 4·240π − 4·90π"
tex = '10(3744+64\pi-144\pi) + 4(1800-144\pi) + 4\cdot240\pi - 4\cdot90\pi'

[area]
value = 14880.38898038469
exact = "(3744 − 116π) + (1960 + 160π) + 336π + 1944 + (1800 − 144π) + 720 + 600π + 64π + 600π"
tex = '(3744-116\pi)+(1960+160\pi)+336\pi+1944+(1800-144\pi)+720+1264\pi'
+++

A housing built up from four parts. The base is a plate from z = 0 to z = 10 whose outline is the rectangle from (−40, −25) to (40, 25) with its four corners rounded to radius 8 (each rounding is a quarter circle centered 8 in from both sides). On it, from z = 10 to z = 14, stands a raised plate, the rectangle from (−30, −15) to (30, 15). A round hole of radius 12 about the z axis goes straight through both, from z = 0 to z = 14. Four tubes of outer radius 5 stand on the base with their axes vertical through (x, y) = (±34, ±19), from z = 10 up to z = 25, each with a bore of radius 3 along its axis that runs on down through the base to z = 0. The tubes clear the raised plate and the base's rounded corners.

**As a set:**

$$\bigl(\mathrm{Base}\cup\mathrm{Plate}\setminus\{\rho\le12\}\bigr)\cup\bigcup_{\pm,\pm}\{\rho_{(\pm34,\pm19)}\le5\}\times[10,25]\ \setminus\ \bigcup_{\pm,\pm}\{\rho_{(\pm34,\pm19)}\le3\}\times[0,25]$$
