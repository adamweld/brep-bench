"""A chain of three balls halved through their centres. The chain of boolean/chain-of-three-balls,
cut by the plane z = 0 through all three centres: keep the part with z ≥ 0."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused, other_case


def build():
    chain = other_case("boolean/chain-of-three-balls").build()
    # Everything below z = 0, with room to spare round the chain.
    below = BRepPrimAPI_MakeBox(gp_Pnt(-20, -20, -20), gp_Pnt(30, 30, 0)).Shape()
    cut = BRepAlgoAPI_Cut(chain, below)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the half-space")
    return cut.Shape()
