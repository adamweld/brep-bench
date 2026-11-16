"""A plate's boss deleted: the plate's top closes over where it stood. A plate, the cylinder of
radius 8 on the z axis from z = −4 to z = 0, with a boss standing on it, the cylinder of radius 3
from z = 0 to z = 8.

Then edit it directly: delete the boss's side and its top. Deleting a face takes it out of the solid
and closes the gap it leaves with the faces round it, each run on along its own surface until they
meet. The two faces meet the rest of the solid only in the circle of radius 3 at the boss's foot,
which lies whole in the plate's top, so that face closes over the circle and nothing else changes.

The result is the plate, the cylinder of radius 8 from z = −4 to z = 0."""
from bench import face_through
from edit import cylinder, deleted, united


def build():
    solid = united(cylinder((0, 0, -4), 8, 4), cylinder((0, 0, 0), 3, 8))
    return deleted(solid, [face_through(solid, (0, -3, 4)), face_through(solid, (0, 0, 8))])
