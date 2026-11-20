+++
title = "A torus less the post its core circle lies on"
tags = ["boolean", "cut", "analytic", "torus", "cylinder"]
origin = "OCCT tests/boolean/boptuc_simple/ZL2"

[volume]
value = 43.667207809143825
exact = "Pappus on the tube's half disc: area π/2, centroid 4a/3π = 4/(3π) outside the core circle, so 2π(4 + 4/3π)·π/2 with a = 1"
tex = '\pi^2 a^2 R + \tfrac{4}{3}\pi a^3,\quad R = 4,\ a = 1'

[area]
value = 141.78868828051073
exact = "outer half of the torus's wall 2π(R + 2a/π)·πa = 2π²aR + 4πa², plus the post's wall where it cuts the tube: 2πR·2a"
tex = '2\pi^2 aR + 4\pi a^2 + 4\pi R a'
+++

A post of radius 4 on the z axis, from z = −3 to z = 3.

A torus about the z axis, centred on the origin: its core circle has radius 4 and its tube radius 1. The core circle lies on the post's wall at mid-height, and the whole tube is clear of the post's ends. Cut the post away from the torus.

The post's wall cuts the tube through its middle, at the core circle, so half the tube is lost: what is left is the outer half of the torus, a ring of half-round section whose flat inner side is a strip of the post's wall, 2 high.

**As a set:**

$$\{\,(\rho - 4)^2 + z^2 \le 1\,\} \setminus \{\,\rho \le 4,\ |z| \le 3\,\},\quad \rho = \sqrt{x^2 + y^2}$$
