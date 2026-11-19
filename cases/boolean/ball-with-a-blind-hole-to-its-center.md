+++
title = "A ball with a blind hole drilled from below to its center"
tags = ["boolean", "cut", "analytic", "sphere", "cylinder", "hole"]
origin = "OCCT mesh/bug29751"

[volume]
value = 3912.507390593155
exact = "4πR³/3 − [π r² h + π(R−h)²(2R+h)/3], R = 10, r = 3, h = √(R²−r²) = √91"
tex = '\tfrac43\pi R^3 - \Bigl[\pi r^2 h + \tfrac{\pi}{3}(R-h)^2(2R+h)\Bigr],\ h=\sqrt{R^2-r^2}'

[area]
value = 1435.7838452573924
exact = "4πR² − 2πR(R−h) + 2πr h + π r²"
tex = '4\pi R^2 - 2\pi R(R-h) + 2\pi r h + \pi r^2'
+++

A ball of radius 10 centered on the origin. Remove from it the cylinder of radius 3 about the z axis that runs from z = −15, below the ball, up to z = 0, the ball's center. This leaves a blind hole of radius 3 entering the ball at its south pole and ending in a flat floor at the center. The hole's wall meets the sphere in a circle at z = −√91.

**As a set:**

$$B_{10}\setminus\bigl(\{\rho\le3\}\times[-15,0]\bigr)$$
