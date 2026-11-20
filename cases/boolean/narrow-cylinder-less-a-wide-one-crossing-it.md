+++
title = "A narrow cylinder less the wide one it crosses"
tags = ["boolean", "cut", "analytic", "cylinder", "pieces"]
origin = "OCCT tests/boolean/boptuc_simple/ZE3"

[volume]
value = 78.64662935796292
exact = "π·2²·14 less the common part of the two cylinders, 4∫ from −2 to 2 of √((2² − z²)(4² − z²)) dz = 97.2825592430655, an elliptic integral"
tex = '\pi r^2 L - 4\int_{-r}^{r}\sqrt{(r^2 - z^2)(R^2 - z^2)}\,dz,\quad r = 2,\ R = 4,\ L = 14'

[area]
value = 133.14747716546089
exact = "the narrow cylinder's own 2π·2·14 + 2π·2², less its wall inside the wide one, ∫ from 0 to 2π of 2r√(R² − r² sin²ψ) dψ = 93.9175813977234, plus the wide cylinder's wall inside the narrow one, 2∫ from −φ₀ to φ₀ of 2R√(r² − R² sin²φ) dφ = 26.0031287334375, sin φ₀ = r/R"
tex = '2\pi r L + 2\pi r^2 - \int_0^{2\pi} 2r\sqrt{R^2 - r^2\sin^2\psi}\,d\psi + 2\int_{-\varphi_0}^{\varphi_0} 2R\sqrt{r^2 - R^2\sin^2\varphi}\,d\varphi'
+++

A wide cylinder of radius 4 on the x axis, from x = −5 to x = 5, and a narrow one of radius 2 and length 14 on the y axis, from y = −7 to y = 7. The axes cross at right angles at the origin. The narrow cylinder passes right through the wide one, and each of its ends sticks out beyond the wall. Cut the wide one away from the narrow one.

What is left is the two ends of the narrow cylinder, each from the wide cylinder's wall out to a flat end at y = ±7, and each with a saddle-shaped concave face where the wide cylinder's wall was: two separate pieces. Build it all as one result and measure it whole.

**As a set:**

$$\{\,x^2 + z^2 \le 4,\ |y| \le 7\,\} \setminus \{\,y^2 + z^2 \le 16,\ |x| \le 5\,\}$$
