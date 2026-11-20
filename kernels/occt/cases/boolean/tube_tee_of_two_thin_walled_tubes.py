"""A tee of two thin-walled tubes. Main tube along x from −20 to 20, outer radius 5, bore 4; stub
along y from 0 to 10 at x = z = 0, outer radius 3, bore 2. The outer cylinders joined, less the
bores joined."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    main = lambda r: BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(-20, 0, 0), gp_Dir(1, 0, 0)), r, 40).Shape()
    stub = lambda r: BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 1, 0)), r, 10).Shape()
    outer = BRepAlgoAPI_Fuse(main(5), stub(3))
    bores = BRepAlgoAPI_Fuse(main(4), stub(2))
    if not (outer.IsDone() and bores.IsDone()):
        raise Refused("BRepAlgoAPI_Fuse failed")
    cut = BRepAlgoAPI_Cut(outer.Shape(), bores.Shape())
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut failed")
    return cut.Shape()
