"""A cone frustum's ends moved, the top out and the bottom in. A cone frustum about the z axis, of
radius 6 at its flat bottom in the plane z = 0 and radius 3 at its flat top in the plane z = 6; its
cone's apex would be at z = 12. Its faces are the side and two flat disks.

Then edit the solid directly: move the top 3 outward along its normal, up to the plane z = 9, and
the bottom 2 inward along its normal, up to the plane z = 2. The cone runs on to meet them, so the
top shrinks to radius 3/2 and the bottom to radius 5.

So the result is the frustum of the same cone from z = 2, radius 5, to z = 9, radius 3/2."""
from bench import face_through
from edit import frustum, moved


def build():
    solid = frustum((0, 0, 0), 6, 3, 6)
    top = face_through(solid, (1, 1, 6))
    bottom = face_through(solid, (1, 1, 0))
    return moved(solid, [(top, 3), (bottom, -2)])
