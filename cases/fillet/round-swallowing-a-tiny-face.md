+++
title = "A broken edge rounded over"
tags = ["fillet", "box", "planar", "cylinder", "vanishing-face", "small"]
origin = "brep-conformance 1.0.0, fillets/round-swallowing-a-tiny-face"

[volume]
value = 498.9269908169872
exact = "5(100 − (1 − π/4)r²), r = 1"
tex = '5\left(100 - \left(1 - \tfrac{\pi}{4}\right)r^2\right),\quad r = 1'

[area]
value = 397.42477796076935
exact = "2(100 − (1 − π/4)r²) + 5(40 − 2r + πr/2), r = 1"
tex = '2\left(100 - \left(1 - \tfrac{\pi}{4}\right)r^2\right) + 5\left(40 - 2r + \tfrac{\pi}{2}r\right),\quad r = 1'
+++

A block from (0, 0, 0) to (10, 10, 5) whose vertical edge at x = 10, y = 10 has been broken by a
narrow flat at 45°: the prism from z = 0 to z = 5 on the pentagon (0, 0), (10, 0), (10, 9.9999),
(9.9999, 10), (0, 10). The flat is 0.0001√2 ≈ 0.00014 wide. Round the two edges where the flat
meets the block's sides, together, with a fillet of radius 1.

A ball of radius 1 cannot touch so narrow a flat: rolled into that corner it touches the sides
x = 10 and y = 10, 1 back from where they would meet, and passes 0.41 clear of the flat. So the
round runs from one side to the other and the flat is gone; the solid is the block with that
vertical edge rounded to radius 1, a quarter cylinder about the line x = 9, y = 9.

**As a set:**

$$\bigl([0, 10]^2 \setminus ((9, 10]^2 \setminus \{\, (x - 9)^2 + (y - 9)^2 \le 1 \,\})\bigr) \times [0, 5]$$
