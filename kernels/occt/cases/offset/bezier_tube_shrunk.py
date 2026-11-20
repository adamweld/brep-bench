"""A tube swept along a Bezier S, shrunk by 1. The tube swept by a disk of radius 2 along the cubic
Bezier curve with control points (0, 0, 0), (12, 0, 0), (12, 12, 0) and (24, 12, 0): an S in the
plane z = 0, of length about 27.73, whose least radius of curvature, about 12.5, is far more than
the tube's radius. At each point of the curve the disk lies in the plane perpendicular to the
curve and is centered on it. The tube's faces are its side, the surface of every point 2 from the
curve, and two flat disks of radius 2 at its ends, perpendicular to the curve there: in the plane
x = 0 at the start and in the plane x = 24 at the end."""
from offset import offset
from turned import tube

SPINE = [(0, 0, 0), (12, 0, 0), (12, 12, 0), (24, 12, 0)]
RADIUS = 2
DISTANCE = -1


def build():
    return offset(tube(SPINE, RADIUS, "the tube"), DISTANCE)
