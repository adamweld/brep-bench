"""A bar of pentagonal section mitred around an L-shaped ring. A bar swept around the outside of an
L-shaped outline, mitred at each of its six corners, one of which is concave.

The path: the closed outline in the xy plane through (0, 0, 0), (40, 0, 0), (40, 20, 0),
(20, 20, 0), (20, 40, 0) and (0, 40, 0), and back to the first, run in that order, counterclockwise.
Five of its corners turn left, and one, at (20, 20, 0), turns right.

The section stands square to the path. It is given by s, the distance outward from the path (to its
right as it is run, away from the L), and z: the polygon through (s, z) = (0, 0), (6, 0), (6, 3),
(3, 5), (0, 5). On the side y = 0 the point (s, z) of the section is at (x, -s, z).

Each side of the outline is the section swept straight along it, and at each corner the two sides are
mitred: each is cut by, and they meet in, the vertical plane through the corner at 45° to both
sides. The path is closed, so the bar has no ends: it is a ring."""
from OCP.gp import gp_Dir

from corner import MITRE, polyline, swept

SECTION = [(0, 0), (6, 0), (6, 3), (3, 5), (0, 5)]  # (s outward, z)


def build():
    # Run the outline from the middle of its side y = 0, where the section stands: there, outward
    # is -y, so the point (s, z) is at (20, -s, z).
    path = polyline([(20, 0, 0), (40, 0, 0), (40, 20, 0), (20, 20, 0), (20, 40, 0), (0, 40, 0),
                     (0, 0, 0)], closed=True)
    section = polyline([(20, -s, z) for s, z in SECTION], closed=True)
    return swept(path, section, MITRE, binormal=gp_Dir(0, 0, 1))  # the section kept upright
