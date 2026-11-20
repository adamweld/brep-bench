+++
title = "A coil of round wire in five helical stretches, narrowing in steps, each of its own pitch"
tags = ["sweep", "helix", "conical", "spring", "miter", "unsettled"]
origin = "OCCT tests/helix/standard/E1"
settled = false
+++

A round wire of radius 1 coiled about the z axis along a centre line of five helical stretches, one
after another: wide, closing in, narrow, opening out, wide again. Every stretch is a helix about the
z axis, turning right-handed (from +x toward +y), whose radius ρ and height z both change in
proportion to the angle turned. The coil starts at (12, 0, 0).

| stretch | turns | radius ρ | rise | pitch (rise per turn) |
|---|---|---|---|---|
| 1 | 1½ | 12 throughout | 7.5 | 5 |
| 2 | 1¼ | 12 closing to 8 | 3.125 | 2.5 |
| 3 | 2 | 8 throughout | 12 | 6 |
| 4 | 1 | 8 opening to 12 | 5 | 5 |
| 5 | ¾ | 12 throughout | 3 | 4 |

So the centre line is 6½ turns in all, rising 30.625, and ends at (−12, 0, 30.625). Each stretch
starts where the one before ends, but with a different pitch, so the centre line has a slight corner
at each of the four joints, where its direction turns by a few degrees; the stretches that change
radius (2 and 4) also lean in or out there.

The wire is the disk of radius 1, centred on the centre line and square to it, carried along each
stretch with it, with flat ends square to the centre line at the start and the end. At each joint the
two stretches are mitered: each is the wire swept along its own helix, run a little past the joint,
and cut by the plane through the joint whose normal is the sum of the two stretches' unit directions
there, and they meet in it.

No two parts of the centre line that are not near each other along it come within 2.6 of one
another, so the 2-thick wire does not touch itself, even between the tight coils of stretch 2.

The case has no closed form: the length of a wire like this is a definite integral, and the mitered
joints shift the volume and area a little from it.
