"""A round rib poking through a block's leaning side. A block whose side leans: everything with 0 ≤ y ≤ 30, 0 ≤ z ≤ 20 and 0 ≤ x ≤ 40 − z/5. Its side
x = 40 − z/5 leans in as it rises, from x = 40 at the bottom to x = 36 at the top; its top face is
the rectangle 0 ≤ x ≤ 36, 0 ≤ y ≤ 30 at z = 20.

A round rib runs along the block's top near that side. Its section, in planes perpendicular to y, is
the part of the disc of radius 8 about (x, z) = (32, 20) that lies above the line z = 18: a little
more than the upper half of the disc. It is swept along y from y = 9 to y = 21, so its two ends are
flat, in the planes y = 9 and y = 21. The rib sticks out above the top face, and its lower part
crosses the leaning side: between z = 18 and z = 20 the side lies inside the rib, and the rib's flat
underside at z = 18 shows beyond it, from x = 36.4 to 32 + √60 ≈ 39.75. The block and the rib are
one solid."""
import math

from ribs import fuse, polygon, prism, segment_xz


def build():
    # The block: its section in the plane y = 0, the side x = 40 - z / 5 leaning in, run along y to 30.
    block = prism(polygon([(0, 0, 0), (40, 0, 0), (36, 0, 20), (0, 0, 20)]), 0, 30, 0)
    # The rib: the section above the line z = 18 of the disc about (x, z) = (32, 20), run from y = 9 to 21.
    tilt = math.degrees(math.asin(1 / 4))
    rib = prism(segment_xz(32, 20, 8, -tilt, 180 + tilt, y=9), 0, 12, 0)
    return fuse(block, rib, "the rib and the block")
