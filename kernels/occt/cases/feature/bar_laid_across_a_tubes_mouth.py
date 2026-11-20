"""A bar laid across the mouth of a tube. The tube 6 <= rho <= 14, 0 <= z <= 10 about the z axis;
the bar |x| <= 20, |y| <= 4 in the plane z = 10 (the tube's top face), extruded up 5 and joined."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    axis = gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1))
    outer = BRepPrimAPI_MakeCylinder(axis, 14, 10).Shape()
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -1), gp_Dir(0, 0, 1)), 6, 12).Shape()
    tube = BRepAlgoAPI_Cut(outer, bore)
    if not tube.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the bore")
    bar = BRepPrimAPI_MakeBox(gp_Pnt(-20, -4, 10), gp_Pnt(20, 4, 15)).Shape()
    fuse = BRepAlgoAPI_Fuse(tube.Shape(), bar)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the bar and the tube")
    fuse.SimplifyResult()
    return fuse.Shape()
