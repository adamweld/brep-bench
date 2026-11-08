"""A core drill pushed along its leaning axis through a plate. A plate, the box from (−10, −10, 0)
to (10, 10, 4), with an annular cut made right through it by a core drill pushed along its own
leaning axis.

The drill is a tube: everything between 1.5 and 2.5 from its axis, which is the line through (0, 0,
2) along the direction u = (sin 30°, 0, cos 30°), leaning 30° from upright toward +x. As placed,
the tube runs along its axis from 18 to 6 before (0, 0, 2), its ends square to its axis, wholly
below the plate.

It slides along u by 24, to run from 6 to 18 past (0, 0, 2), wholly above the plate. Cut from the
plate everything the tube passes through.

The tube's shadow along the push is an annulus, so it sweeps two oblique cylinders through the
plate. The core inside the inner one is cut free of the plate and stays: the result is two separate
solids, the plate with its oblique hole and the core, an oblique cylinder of radius 1.5 cut by the
planes z = 0 and z = 4."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused
from sweep import cut, slid

PLATE = ((-10, -10, 0), (10, 10, 4))
CENTRE = (0, 0, 2)
LEAN = 30  # degrees from upright, toward +x
OUTER, INNER = 2.5, 1.5
FROM, TO = -18, -6  # where the tube starts and ends along its axis, past CENTRE
PUSH = 24


def build():
    u = (math.sin(math.radians(LEAN)), 0, math.cos(math.radians(LEAN)))
    foot = gp_Pnt(*(c + FROM * d for c, d in zip(CENTRE, u)))
    axis = gp_Ax2(foot, gp_Dir(*u))
    outer = BRepPrimAPI_MakeCylinder(axis, OUTER, TO - FROM).Shape()
    inner = BRepPrimAPI_MakeCylinder(axis, INNER, TO - FROM).Shape()
    tube = BRepAlgoAPI_Cut(outer, inner)
    if not tube.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the tube's bore")
    # Pushed along its own axis, the tube's two cylinders slide along themselves; its flat ends,
    # square to the push, lead or trail all over.
    swept = slid(tube.Shape(), tuple(PUSH * d for d in u))
    return cut(BRepPrimAPI_MakeBox(gp_Pnt(*PLATE[0]), gp_Pnt(*PLATE[1])).Shape(), swept)
