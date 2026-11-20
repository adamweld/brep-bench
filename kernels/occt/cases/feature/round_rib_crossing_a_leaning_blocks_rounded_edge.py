"""A round rib crossing a leaning block's rounded top edge. A block whose side leans, with its top edge rounded. Its section in the xz plane is the quadrilateral
with corners (x, z) = (0, 0), (40, 0), (30, 24) and (0, 24), swept along y from y = 0 to y = 30. The
side from (40, 0) to (30, 24) is the plane 12x + 5z = 480, which leans in as it rises. The edge
between that side and the top, at (30, 24), is rounded to radius 6: a cylinder
about the line x = 26, z = 18 (parallel to y), touching the top at x = 26 and the leaning side at
(26 + 72/13, 18 + 30/13) ≈ (31.54, 20.31).

A round rib lies along the block's top and goes down over the rounded edge. Its section, in planes
perpendicular to y, is the part of the disc of radius 8 about (x, z) = (28, 26) that lies above the
line z = 22 (a little more than the upper half of the disc), swept along y over the block's whole
length, from y = 0 to y = 30, so its two ends lie flush with the block's. It covers the top face from
x = 28 − √60 ≈ 20.25 on, and the rounded edge down to z = 22, where its flat underside shows
beyond the round, from x = 26 + √20 ≈ 30.47 on the round to x = 28 + √48 ≈ 34.93. Below z = 22 the
rounded edge and the leaning side are left as they were. The block and the rib are one solid."""
import math

from ribs import face, fuse, prism, segment_xz


def build():
    # The block's section in the plane y = 0: the leaning side's rounded edge is the arc about
    # (x, z) = (26, 18), from the leaning side's tangent point to the top's.
    t = (26 + 72 / 13, 0, 18 + 30 / 13)
    m = math.radians((90 + math.degrees(math.atan2(5, 12))) / 2)
    mid = (26 + 6 * math.cos(m), 0, 18 + 6 * math.sin(m))
    section = face([((0, 0, 0), (40, 0, 0)), ((40, 0, 0), t), (t, mid, (26, 0, 24)),
                    ((26, 0, 24), (0, 0, 24)), ((0, 0, 24), (0, 0, 0))])
    block = prism(section, 0, 30, 0)
    # The rib: the part of the disc about (x, z) = (28, 26) above the line z = 22, over the whole length.
    tilt = math.degrees(math.asin(0.5))
    rib = prism(segment_xz(28, 26, 8, -tilt, 180 + tilt), 0, 30, 0)
    return fuse(block, rib, "the rib and the block")
