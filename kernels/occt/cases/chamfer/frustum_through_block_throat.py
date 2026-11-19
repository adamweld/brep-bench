"""A cone frustum through a block, both junctions chamfered by their throat. A block from
(−10, −10, 0) to (10, 10, 10), joined to a cone frustum on the z axis that runs right through it:
radius 5.5 at z = −2 narrowing to 1.5 at z = 14, so its radius is ρ = 5 − z/4, 5 where it meets
the block's bottom face and 2.5 where it meets the top. Both junctions are concave circular edges:
the circle of radius 2.5 at z = 10 and the circle of radius 5 at z = 0.

Chamfer both edges by a throat of 0.5, as a weld is sized. A concave chamfer adds material: it
fills the groove between the frustum and the block's face. In the plane through the z axis (the
plane square to the edge), the chamfer's section is an isosceles triangle: its two legs, along the
block's face and along the frustum's side from the edge, are equal, and its third side, the
chamfer's face, lies 0.5 from the edge. The legs are d = 0.5/cos(θ/2), θ being the angle of the
groove between the face and the side, which is not the same at the two junctions:

- at the top, the side leans in over the face: θ = 90° + atan(1/4) ≈ 104.04°, and d ≈ 0.81246,
  so the chamfer meets the top face on the circle of radius 2.5 + d and the side on the circle at
  z = 10 + 4d/√17;
- at the bottom, the side leans out under the face: θ = 90° − atan(1/4) ≈ 75.96°, and
  d ≈ 0.63435, so the chamfer meets the bottom face on the circle of radius 5 + d and the side on
  the circle at z = −4d/√17.

Each chamfer is the cone between its two circles, and the solid is the block and the frustum
joined, with the two rings between them and the cones filled in."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone
from OCP.ChFiDS import ChFiDS_ChamfMode
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused
from chamfer import edge_through

THROAT = 0.5


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(-10, -10, 0), gp_Pnt(10, 10, 10)).Shape()
    frustum = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, -2), gp_Dir(0, 0, 1)), 5.5, 1.5, 16).Shape()
    joined = BRepAlgoAPI_Fuse(block, frustum)
    if not joined.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the frustum to the block")
    joined = joined.Shape()
    chamfer = BRepFilletAPI_MakeChamfer(joined)
    # a chamfer sized by its throat: equal legs, its face THROAT from the edge
    chamfer.SetMode(ChFiDS_ChamfMode.ChFiDS_ConstThroatChamfer)
    chamfer.Add(THROAT, edge_through(joined, (0, 2.5, 10)))  # the top junction
    chamfer.Add(THROAT, edge_through(joined, (0, 5, 0)))  # the bottom junction
    chamfer.Build()
    if not chamfer.IsDone():
        raise Refused("BRepFilletAPI_MakeChamfer could not chamfer the junctions by their throat")
    return chamfer.Shape()
