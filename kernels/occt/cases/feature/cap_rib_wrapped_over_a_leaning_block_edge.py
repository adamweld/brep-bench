"""A cap rib wrapped over a block's leaning top edge. A block whose side leans: everything with 0 ≤ y ≤ 30, 0 ≤ z ≤ 20 and 0 ≤ x ≤ 40 − z/5. Its side
x = 40 − z/5 leans in as it rises, from x = 40 at the bottom to x = 36 at the top; its top face is
the rectangle 0 ≤ x ≤ 36, 0 ≤ y ≤ 30 at z = 20, and the edge between the top and the side is the
line x = 36, z = 20.

A cap rib wraps over that edge. Its profile, in planes perpendicular to y, is the open two-segment
line from (x, z) = (28, 20) on the top face out to (42, 28) and back in to (38, 10) on the leaning
side; the rib fills the space between that line and the block. In the xz plane it is the
quadrilateral with corners (28, 20), (42, 28), (38, 10) and (36, 20), which goes round the edge
(the corner at (36, 20) is the block's own edge, where the rib's angle is 270°). It is swept along y
from y = 8 to y = 22, so its two ends are flat, in the planes y = 8 and y = 22. The block and the
rib are one solid."""
from ribs import fuse, polygon, prism


def build():
    # The block: its section in the plane y = 0, the side x = 40 - z / 5 leaning in, run along y to 30.
    block = prism(polygon([(0, 0, 0), (40, 0, 0), (36, 0, 20), (0, 0, 20)]), 0, 30, 0)
    # The cap: the quadrilateral from the top face over the edge to the leaning side, run from y = 8 to 22.
    cap = prism(polygon([(28, 8, 20), (42, 8, 28), (38, 8, 10), (36, 8, 20)]), 0, 14, 0)
    return fuse(block, cap, "the cap and the block")
