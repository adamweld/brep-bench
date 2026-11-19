"""A blind hole drilled at a compound angle. A block, the box from the origin to (50, 40, 30), with
a round blind hole of radius 4 drilled into its top face aslant, its axis leaning toward neither of
the block's sides.

The hole's axis enters the top face (z = 30) at (25, 20, 30) and runs into the block along the
direction u = (−2, 1, −2)/3, so it makes an angle with the vertical whose cosine is 2/3 (about
48.19°). The hole is drilled 18 along that axis: its floor is the flat disk of radius 4 square to
the axis, centred at (25, 20, 30) + 18u = (13, 26, 18). Its mouth in the top face is an ellipse,
semi-axes 6 along (−2, 1)/√5 and 4 across it. The hole stays inside the block and reaches no other
face.

If you cut the hole with a cylinder, make it run out past the top face."""
from OCP.BRepFeat import BRepFeat_MakeCylindricalHole, BRepFeat_Status
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Ax1, gp_Dir, gp_Pnt

from bench import Refused

ENTRY = (25, 20, 30)
AXIS = (-2 / 3, 1 / 3, -2 / 3)
RADIUS, DEPTH = 4, 18
BACK = 8  # start the hole this far back along the axis, clear above the top face


def build():
    block = BRepPrimAPI_MakeBox(50, 40, 30).Shape()
    start = gp_Pnt(*(e - BACK * a for e, a in zip(ENTRY, AXIS)))
    hole = BRepFeat_MakeCylindricalHole()
    hole.Init(block, gp_Ax1(start, gp_Dir(*AXIS)))
    # Blind: its length is measured from the axis's origin, the start above the face.
    hole.PerformBlind(RADIUS, BACK + DEPTH, False)
    if hole.Status() != BRepFeat_Status.BRepFeat_NoError:
        raise Refused(f"BRepFeat_MakeCylindricalHole could not drill the hole: {hole.Status()}")
    return hole.Shape()
