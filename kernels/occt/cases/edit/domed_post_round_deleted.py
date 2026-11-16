"""A post capped by a ball: the cylinder r 4 (z 0..6) under the cap above z 6 of the ball r 5 about (0, 0, 3), its rim rounded 1; then the round deleted: the domed post."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt
from blend import edges_where, rounded
from edit import deleted, on_circle
def near(solid, p):
    from OCP.BRepExtrema import BRepExtrema_DistShapeShape
    from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeVertex
    from OCP.TopAbs import TopAbs_FACE
    from OCP.TopExp import TopExp_Explorer
    from OCP.TopoDS import TopoDS
    v = BRepBuilderAPI_MakeVertex(gp_Pnt(*p)).Vertex(); best = None
    e = TopExp_Explorer(solid, TopAbs_FACE)
    while e.More():
        f = TopoDS.Face(e.Current()); d = BRepExtrema_DistShapeShape(v, f); d.Perform()
        if best is None or d.Value() < best[0]: best = (d.Value(), f)
        e.Next()
    return best[1]
def build():
    cap = BRepAlgoAPI_Common(BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 3), 5).Shape(), BRepPrimAPI_MakeBox(gp_Pnt(-6, -6, 6), gp_Pnt(6, 6, 10)).Shape()).Shape()
    b = BRepAlgoAPI_Fuse(BRepPrimAPI_MakeCylinder(4, 6).Shape(), cap).Shape()
    r = rounded(b, 1, edges_where(b, on_circle((0, 0, 6), 4)), 'edge')
    return deleted(r, [near(r, (0, -3.9426028923173684, 5.979667149854743))])
