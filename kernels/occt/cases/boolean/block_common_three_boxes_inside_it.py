"""A block in common with three boxes inside it. A block 0 to 10 each way; a body of three separate
boxes, (1,1,1)-(3,3,3), (4,1,1)-(7,2,5) and (5,5,6)-(9,9,8), gathered in one compound. Make the part
common to the block and the compound."""
from OCP.BRep import BRep_Builder
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopoDS import TopoDS_Compound
from OCP.gp import gp_Pnt

from bench import Refused

BOXES = [((1, 1, 1), (3, 3, 3)), ((4, 1, 1), (7, 2, 5)), ((5, 5, 6), (9, 9, 8))]


def build():
    block = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    body, builder = TopoDS_Compound(), BRep_Builder()
    builder.MakeCompound(body)
    for low, high in BOXES:
        builder.Add(body, BRepPrimAPI_MakeBox(gp_Pnt(*low), gp_Pnt(*high)).Shape())
    common = BRepAlgoAPI_Common(block, body)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the block and the body of three boxes")
    return common.Shape()
