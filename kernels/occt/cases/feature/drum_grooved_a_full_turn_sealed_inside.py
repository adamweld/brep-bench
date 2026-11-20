"""A drum with a ring-shaped void turned a full turn inside it. The drum is a cylinder of radius 20
on the z axis, from z = −15 to z = 15. A disk of radius 3 centred at (10, 0, 0) in the plane y = 0 is
turned a full turn about the z axis, sweeping a torus; the torus is removed from the drum."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused

DRUM_RADIUS, HALF_HEIGHT = 20, 15
RING, SECTION = 10, 3


def build():
    drum = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -HALF_HEIGHT), gp.DZ_s()),
                                    DRUM_RADIUS, 2 * HALF_HEIGHT).Shape()
    circle = gp_Circ(gp_Ax2(gp_Pnt(RING, 0, 0), gp_Dir(0, 1, 0)), SECTION)
    wire = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(circle).Edge()).Wire()
    disk = BRepBuilderAPI_MakeFace(wire).Face()
    groove = BRepPrimAPI_MakeRevol(disk, gp.OZ_s(), 2 * math.pi).Shape()
    cut = BRepAlgoAPI_Cut(drum, groove)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the drum and the torus inside it")
    return cut.Shape()
