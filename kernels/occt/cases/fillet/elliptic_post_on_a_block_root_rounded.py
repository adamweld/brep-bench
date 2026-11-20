"""An elliptic post standing on a block, its root rounded. A block from (-30, -30, 0) to
(30, 30, 10), and standing on its top face the elliptic cylinder x^2/12^2 + y^2/6^2 <= 1 from
z = 10 to z = 25, the two fused into one solid. The ellipse where they meet is rounded to radius 2."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.GeomAbs import GeomAbs_CurveType
from OCP.gp import gp, gp_Ax2, gp_Elips, gp_Pnt, gp_Vec

from bench import Refused
from blend import edges_where, rounded


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(-30, -30, 0), gp_Pnt(30, 30, 10)).Shape()
    ellipse = BRepBuilderAPI_MakeEdge(gp_Elips(gp_Ax2(gp_Pnt(0, 0, 10), gp.DZ_s()), 12, 6)).Edge()
    face = BRepBuilderAPI_MakeFace(BRepBuilderAPI_MakeWire(ellipse).Wire()).Face()
    post = BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, 15)).Shape()
    fuse = BRepAlgoAPI_Fuse(block, post)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the post to the block")
    part = fuse.Shape()
    root = edges_where(part, lambda p: abs(p.Z() - 10) < 1e-6
                       and abs(p.X() ** 2 / 144 + p.Y() ** 2 / 36 - 1) < 1e-6)
    if len(root) != 1:
        raise Refused(f"{len(root)} root edges")
    return rounded(part, 2, root, "the post's root")
