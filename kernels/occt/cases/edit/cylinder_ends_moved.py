"""A cylinder's ends moved, the top out and the bottom in. A cylinder of radius 5 about the z axis,
from the flat bottom in the plane z = 0 to the flat top in the plane z = 10. Its faces are the side
and two flat disks.

Then edit the solid directly: move the top 3 outward along its normal, up to the plane z = 13, and
the bottom 2 inward along its normal, up to the plane z = 2. The side runs on to meet them.

So the result is the cylinder of radius 5 from z = 2 to z = 13."""
from bench import face_through
from edit import cylinder, moved


def build():
    solid = cylinder((0, 0, 0), 5, 10)
    top = face_through(solid, (1, 1, 10))
    bottom = face_through(solid, (1, 1, 0))
    return moved(solid, [(top, 3), (bottom, -2)])
