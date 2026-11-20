+++
title = "A drafted post on a dome, its foot rounded"
tags = ["draft", "fillet", "cone", "sphere", "torus", "concave", "boolean"]
settled = false
origin = "OCCT tests/draft/angle/F1"
+++

A hemisphere with a drafted post standing in it, and the post's foot rounded where it leaves the
dome. It has no closed form that has been checked.

The dome: everything within 15 of the origin with z ≥ 0, a hemisphere whose flat face is the disk
z = 0 and whose pole is (0, 0, 15).

The post: the solid of revolution about the z axis from z = 0 to z = 22, its radius 4 at z = 15 and
narrowing as it rises, ρ = 4 − (z − 15)·tan 3°. That is a cylinder of radius 4 drafted by 3° (pull
direction +z, neutral plane z = 15) into a cone; its radius is a little over 4 where it leaves the
dome, a little below the pole's height, and about 3.63 at its flat top, z = 22. The post is united
with the dome.

The foot: the edge where the post's cone leaves the dome's sphere, a circle, is rounded with a
constant radius of 1.5, adding material in the concave corner between dome and post: a ball of
radius 1.5 rolls in that corner, outside the dome's sphere and outside the post's cone (in the air
above the dome beside the post), touching both; the round is the material between the ball and the
corner, bounded by the ball's two circles of contact, one on the sphere and one on the cone.
Nothing else is rounded.
