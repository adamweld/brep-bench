"""A plate's three edges at a corner rounded with unequal radii, creased where they meet. The box
from the origin to (20, 12, 4); at the corner (20, 0, 4) the edge along x (y = 0, z = 4) is rounded
to radius 1, the edge along y (x = 20, z = 4) to radius 2, and the upright edge (x = 20, y = 0) to
radius 1.5, each running the whole length of its edge.

Finding: one BRepFilletAPI_MakeFillet over the three edges closes the corner with a patch of its
own, and its solid differs from the creased one the words describe (by about 3e-4), and rounded one
after another OCCT's second round already takes the upright edge with it. So the solid is built as
the words say: the plate rounded at each edge on its own, and the three rounded plates' common part."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from bench import Refused
from blend import edges_where, rounded

TOLERANCE = 1e-7
ROUNDS = (
    (1, lambda p: abs(p.Y()) < TOLERANCE and abs(p.Z() - 4) < TOLERANCE),
    (2, lambda p: abs(p.X() - 20) < TOLERANCE and abs(p.Z() - 4) < TOLERANCE),
    (1.5, lambda p: abs(p.X() - 20) < TOLERANCE and abs(p.Y()) < TOLERANCE),
)


def build():
    plate = BRepPrimAPI_MakeBox(20, 12, 4).Shape()
    solid = plate
    for radius, test in ROUNDS:
        one = rounded(plate, radius, edges_where(plate, test), "one of the three edges")
        common = BRepAlgoAPI_Common(solid, one)
        if not common.IsDone():
            raise Refused("BRepAlgoAPI_Common could not intersect the rounded plates")
        solid = common.Shape()
    return solid
