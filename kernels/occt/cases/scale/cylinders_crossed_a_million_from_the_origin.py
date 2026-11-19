"""Two cylinders crossed a million from the origin. Two cylinders of radius 3 and length 20, both
centred on c = (600000, −800000, 0), one along (2, 3, 6)/7 and one along (3, −6, 2)/7, each running
10 either way from c. Make the solid common to both: a Steinmetz bicylinder."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

CENTRE = (600000, -800000, 0)
RADIUS, HALF_LENGTH = 3, 10


def cylinder(direction):
    """The cylinder along `direction` from CENTRE − 10u to CENTRE + 10u."""
    u = gp_Dir(*direction)
    base = gp_Pnt(*(c - HALF_LENGTH * d for c, d in zip(CENTRE, (u.X(), u.Y(), u.Z()))))
    return BRepPrimAPI_MakeCylinder(gp_Ax2(base, u), RADIUS, 2 * HALF_LENGTH).Shape()


def build():
    common = BRepAlgoAPI_Common(cylinder((2, 3, 6)), cylinder((3, -6, 2)))
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the two cylinders")
    return common.Shape()
