"""Two cone frustums base to base (3 -> 5 over z 0..4, 5 -> 2 over z 4..8), their ridge rounded 1; then the round deleted: the two frustums."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt
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
    b = BRepAlgoAPI_Fuse(BRepPrimAPI_MakeCone(3, 5, 4).Shape(), BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 4), gp_Dir(0, 0, 1)), 5, 2, 4).Shape()).Shape()
    r = rounded(b, 1, edges_where(b, on_circle((0, 0, 4), 5)), 'edge')
    return deleted(r, [near(r, (0, -4.825138920703175, 3.984232786315833))])
