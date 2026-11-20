"""A triangular rib crossing a leaning block's rounded top edge. A block whose side leans, with its top edge rounded. Its section in the xz plane is the quadrilateral
with corners (x, z) = (0, 0), (40, 0), (30, 24) and (0, 24), swept along y from y = 0 to y = 30. The
side from (40, 0) to (30, 24) is the plane 12x + 5z = 480, which leans in as it rises. The edge
between that side and the top, at (30, 24), is rounded to radius 6: a cylinder
about the line x = 26, z = 18 (parallel to y), touching the top at x = 26 and the leaning side at
(26 + 72/13, 18 + 30/13) ≈ (31.54, 20.31).

A triangular rib lies against the leaning side, across the start of the round. Its section, in
planes perpendicular to y, is the triangle with corners (x, z) = (28, 18), (38, 18) and (33, 22),
swept along y over the block's whole length, from y = 0 to y = 30. It comes out of the leaning side
at z = 18 (where the side is at x = 32.5) with its flat underside, and its two sloping faces meet
the round: the left one, from (28, 18) to (33, 22), crosses the leaning side and then the round
near the point where they touch. The rounded edge above the rib and the leaning side below it are
left as they were. The block and the rib are one solid."""
import math

from ribs import face, fuse, polygon, prism


def build():
    # The block's section in the plane y = 0: the leaning side's rounded edge is the arc about
    # (x, z) = (26, 18), from the leaning side's tangent point to the top's.
    t = (26 + 72 / 13, 0, 18 + 30 / 13)
    m = math.radians((90 + math.degrees(math.atan2(5, 12))) / 2)
    mid = (26 + 6 * math.cos(m), 0, 18 + 6 * math.sin(m))
    section = face([((0, 0, 0), (40, 0, 0)), ((40, 0, 0), t), (t, mid, (26, 0, 24)),
                    ((26, 0, 24), (0, 0, 24)), ((0, 0, 24), (0, 0, 0))])
    block = prism(section, 0, 30, 0)
    rib = prism(polygon([(28, 0, 18), (38, 0, 18), (33, 0, 22)]), 0, 30, 0)
    return fuse(block, rib, "the rib and the block")
