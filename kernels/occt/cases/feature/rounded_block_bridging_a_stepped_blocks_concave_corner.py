"""A block with a rounded corner bridging a stepped block's concave corner and leaning side. A stepped block whose side leans, with its top edge rounded. Its section in the xz plane is the
hexagon with corners (x, z) = (0, 0), (40, 0), (30, 24), (14, 24), (14, 12) and (0, 12), swept along y
from y = 0 to y = 30: a low ledge, z = 12 for x from 0 to 14, then a step up at x = 14 to the top,
z = 24. The step makes a concave corner at (14, 12). The side from (40, 0) to (30, 24) is the plane
12x + 5z = 480, which leans in as it rises. The edge between that side and the top, at (30, 24), is
rounded to radius 6: a cylinder about the line x = 26, z = 18 (parallel to y), touching the top at
x = 26 and the leaning side at (26 + 72/13, 18 + 30/13) ≈ (31.54, 20.31).

A block bridges the step and the leaning side. Its section is the rectangle 8 ≤ x ≤ 34,
10 ≤ z ≤ 22 with its corner at (8, 22) rounded by a quarter circle of radius 4 about (12, 18), swept
along y over the whole length, from y = 0 to y = 30. It fills the space over the ledge from x = 8 to
the step, up to z = 22, covering the concave corner (14, 12); and on the other side its right face
x = 34 comes out of the leaning side at z = 14.4 and runs up to z = 22, beyond the round, where its
top shows from x = 26 + √20 ≈ 30.47 to x = 34. The step above z = 22 and the rounded edge above
that are left as they were. The stepped block and the rib are one solid."""
import math

from ribs import face, fuse, prism

S = 4 / 2 ** 0.5


def build():
    # The stepped block's section in the plane y = 0; the leaning side's rounded edge is the arc
    # about (x, z) = (26, 18) from the leaning side's tangent point to the top's.
    t = (26 + 72 / 13, 0, 18 + 30 / 13)
    m = math.radians((90 + math.degrees(math.atan2(5, 12))) / 2)
    mid = (26 + 6 * math.cos(m), 0, 18 + 6 * math.sin(m))
    section = face([((0, 0, 0), (40, 0, 0)), ((40, 0, 0), t), (t, mid, (26, 0, 24)),
                    ((26, 0, 24), (14, 0, 24)), ((14, 0, 24), (14, 0, 12)), ((14, 0, 12), (0, 0, 12)),
                    ((0, 0, 12), (0, 0, 0))])
    block = prism(section, 0, 30, 0)
    # The bridging block: the rectangle with its (8, 22) corner rounded about (12, 18).
    outline = face([((8, 0, 10), (34, 0, 10)), ((34, 0, 10), (34, 0, 22)), ((34, 0, 22), (12, 0, 22)),
                    ((12, 0, 22), (12 - S, 0, 18 + S), (8, 0, 18)), ((8, 0, 18), (8, 0, 10))])
    return fuse(block, prism(outline, 0, 30, 0), "the bridging block and the stepped block")
