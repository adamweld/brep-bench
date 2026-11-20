"""The bar the feature cases `a drafted bar cut by a U-shaped groove` and `... notched by a rounded
pocket` start from: a box 20 × 30 × 10 with its x = 20 side drafted 10° and its two long top edges
rounded to 2."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Pnt

from bench import Refused, face_through
from draft import drafted

WIDTH, LENGTH, HEIGHT = 20, 30, 10
DRAFT = 10  # degrees, the x = 20 side leaning in as it rises
ROUND = 2
FLOOR = 7


def bar():
    box = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(WIDTH, LENGTH, HEIGHT)).Shape()
    side = face_through(box, (WIDTH, LENGTH / 2, HEIGHT / 2))
    leaning = drafted(box, [side], DRAFT, neutral=(0, 0, 0))
    rounds = BRepFilletAPI_MakeFillet(leaning)
    edges = TopExp_Explorer(leaning, TopAbs_EDGE)
    taken = 0
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        curve = BRepAdaptor_Curve(edge)
        a, b = curve.Value(curve.FirstParameter()), curve.Value(curve.LastParameter())
        # a long top edge: along y, at the top
        if abs(a.Z() - HEIGHT) < 1e-7 and abs(b.Z() - HEIGHT) < 1e-7 and abs(a.X() - b.X()) < 1e-7:
            rounds.Add(ROUND, edge)
            taken += 1
        edges.Next()
    # each long edge is met twice by the explorer, once per face; Add ignores the repeat
    rounds.Build()
    if not rounds.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the bar's top edges")
    return rounds.Shape()
