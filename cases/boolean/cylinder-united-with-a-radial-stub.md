+++
title = "A cylinder united with a narrower cylinder starting on its axis and leaving through its side"
tags = ["boolean", "union", "analytic", "cylinder", "ssi"]
origin = "OCCT boolean/bfuse_complex/P4"

[volume]
value = 1151.1060337837506
exact = "V = πR²·14 + πr²·9 − C, R = 5, r = 2, C = ∫ from −2 to 2 of 2√(r² − y²)·√(R² − y²) dy: for each y the stub takes z from −√(r² − y²) to √(r² − y²) and, inside the wide cylinder, x from 0 to √(R² − y²)"
tex = '\pi R^2\cdot 14 + \pi r^2\cdot 9 - \int_{-r}^{r} 2\sqrt{(r^2-y^2)(R^2-y^2)}\,dy,\quad R = 5,\ r = 2'

[area]
value = 649.4944733278657
exact = "(2πR·14 + 2πR²) − W + πr² + N, with W = ∫ from −2 to 2 of 2√(r² − y²)·R/√(R² − y²) dy the wide wall's patch the stub leaves through, and N = ∫₀^2π r·(9 − √(R² − r²cos²θ)) dθ the stub's wall outside the wide cylinder; the stub's inner end disc lies inside the wide cylinder and is not on the surface"
tex = '2\pi R\cdot 14 + 2\pi R^2 - \int_{-r}^{r} \frac{2R\sqrt{r^2-y^2}}{\sqrt{R^2-y^2}}\,dy + \pi r^2 + \int_0^{2\pi} r\bigl(9 - \sqrt{R^2 - r^2\cos^2\theta}\bigr)\,d\theta,\quad R = 5,\ r = 2'
+++

A cylinder of radius 5 and length 14 along the z axis, from z = −7 to z = 7, and a cylinder of radius 2 and length 9 along the x axis, from x = 0 to x = 9: the narrow one starts at the origin, on the wide one's axis, with its end disc square to the x axis and centred on it, and leaves the wide one through its side, ending 4 beyond the wall. Make their union.

The narrow cylinder's wall is wholly within the wide cylinder's length, since 2 < 7. Its first end disc, at x = 0, lies inside the wide cylinder and is not on the union's surface; the stub stands on the wide cylinder's wall as a round boss 4 long, ending in the flat disc at x = 9.

**As a set:**

$$\{\,\rho \le 5,\ |z| \le 7\,\} \cup \{\,y^2 + z^2 \le 2^2,\ 0 \le x \le 9\,\}$$
