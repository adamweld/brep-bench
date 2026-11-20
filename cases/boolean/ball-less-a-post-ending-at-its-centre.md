+++
title = "A ball less a post that ends at its centre"
tags = ["boolean", "cut", "analytic", "sphere", "cylinder"]
origin = "OCCT tests/boolean/boptuc_simple/ZI8"

[volume]
value = 395.84067435231395
exact = "ball 500π/3, less the post's part inside it: ∫₀³ 2πρ√(25 − ρ²) dρ = 122π/3"
tex = '\tfrac{500\pi}{3} - \tfrac{122\pi}{3}'

[area]
value = 386.41589639154457
exact = "sphere 100π less the cap the post opens (height 1: 10π), plus the cavity's ceiling 9π and its wall 2π·3·4 = 24π"
tex = '100\pi - 10\pi + 9\pi + 24\pi'
+++

A ball of radius 5 centred on the origin.

A post of radius 3 on the z axis, from z = −6 to z = 0: its top face is flat and passes through the ball's centre, and its bottom is clear below the ball. Cut the post away from the ball.

The post's wall meets the ball's surface in a circle at z = −4, where the ball has radius 3. What is left is the whole ball but for a round blind hole opening at the bottom: 4 deep up to z = −4 through the ball's lower cap, then the hole's wall rises to its flat ceiling at z = 0.

**As a set:**

$$\{\,x^2 + y^2 + z^2 \le 25\,\} \setminus \{\,x^2 + y^2 \le 9,\ -6 \le z \le 0\,\}$$
