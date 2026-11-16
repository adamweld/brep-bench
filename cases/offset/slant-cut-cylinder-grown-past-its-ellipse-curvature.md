+++
title = "A cylinder cut by a plane at 45° and grown by 2.5"
tags = ["offset", "grow", "ellipse", "canal", "convex"]

[volume]
value = 1391.4739790096469
exact = "V₀ + S₀d + Md² + 4πd³/3 (Steiner), V₀ = 90π, S₀ = 69π + 9π/cos α, M = 10π + 3π²/2 + J/2, J = ∫₀^{2π} arccos(−sin α cos φ)·3√(1 + tan²α sin²φ) dφ = 36.004515927808, α = 45°, d = 2.5"
tex = 'V_0 + S_0 d + M d^2 + \tfrac{4}{3}\pi d^3,\quad V_0 = 90\pi,\ S_0 = 69\pi + \tfrac{9\pi}{\cos\alpha},\ M = 10\pi + \tfrac{3}{2}\pi^2 + \tfrac{1}{2}J,\ J = \int_0^{2\pi} \arccos(-\sin\alpha\cos\varphi)\,3\sqrt{1 + \tan^2\alpha\sin^2\varphi}\,d\varphi,\ \alpha = 45^\circ,\ d = 2.5'

[area]
value = 656.4086113880463
exact = "S₀ + 2Md + 4πd², with S₀ and M as for the volume"
tex = 'S_0 + 2Md + 4\pi d^2'
+++

A cylinder of radius 3 about the z axis, from its flat bottom at z = −10 up to where the plane
z = x·tan 45° cuts it: the plane crosses the axis at z = 0 and leans at 45° to the bottom, so the
top is the ellipse it cuts from the cylinder, with semi-axes 3 and 3/cos 45°.

Grow it by 2.5 with rounded edges: the solid of every point within 2.5 of it. It is convex, so
each face moves out by 2.5 along its normal, the bottom circle's edge becomes a piece of a torus,
the elliptic edge where the slanted top meets the side becomes a canal surface (the envelope of the
balls of radius 2.5 along the ellipse), and the faces stay tangent to one another.

2.5 is more than the ellipse's least radius of curvature, b²/a = 9/(3/cos 45°) = 2.121320 at the ends of its long axis, so the whole tube of radius 2.5 about the ellipse would cross itself on the ellipse's inner side. The canal face is only the part of that tube between the top's normal and the side's, on the outer side, where it stays smooth: the solid is convex and the formula below holds for every d.

By Steiner's formula for a convex body K grown by d, its volume is V₀ + S₀d + Md² + 4πd³/3 and its
area S₀ + 2Md + 4πd², where V₀ and S₀ are K's volume and area and M is its integrated mean
curvature: the side's 10π (its area 60π times the mean curvature 1/6, the plane's top crossing the
axis at z = 0), plus half of each edge's turning angle integrated along it, 3π²/2 for the bottom
circle (a right angle along 6π) and J/2 for the ellipse, where the angle between the top's normal
and the side's at the point (3 cos φ, 3 sin φ, 3 cos φ·tan α) is arccos(−sin α cos φ) and the
ellipse's length element is 3√(1 + tan²α sin²φ) dφ. J has no elementary closed form; the value
above is it to twelve places (any quadrature agrees to well under the grade).

**As a set:**

$$\{ p : \operatorname{dist}(p, K) \le 2.5 \},\quad K = \{ x^2 + y^2 \le 9,\ -10 \le z \le x\tan 45^\circ \}$$
