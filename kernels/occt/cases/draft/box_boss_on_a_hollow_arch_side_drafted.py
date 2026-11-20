"""A box boss on the arched top of a hollow block, its side drafted. A closed hollow block with an
arched top: the prism along y, from y = 0 to 20, of the region x from 0 to 40 bounded below by
z = 0 and above by the arc of the circle of radius 52 about (20, -40) in the xz plane, less its
inward offset by 2 (x from 2 to 38, y from 2 to 18, z from 2 up to the circle of radius 50 about the
same center). The box with corners (14, 6, 9) and (26, 14, 18) is united with it, filling a slab
of the cavity under the crown and standing out of the arch.

Draft the part of the box's side face y = 14 that stands out above the arch by 5°, with the pull
direction +z and the neutral plane z = 11: it leans toward the box's inside, -y, as it rises."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused, face_through
from draft import drafted

CENTER_Z = -40  # the arch's axis, the line parallel to y through (20, ., -40)


def arched(radius, x_lo, x_hi, y_lo, y_hi, z_lo):
    """The prism along y of the region x from x_lo to x_hi, above z_lo, below the arch of the
    given radius: a box meeting the cylinder about the arch's axis."""
    box = BRepPrimAPI_MakeBox(gp_Pnt(x_lo, y_lo, z_lo), gp_Pnt(x_hi, y_hi, 20)).Shape()
    axis = gp_Ax2(gp_Pnt(20, y_lo, CENTER_Z), gp_Dir(0, 1, 0))
    drum = BRepPrimAPI_MakeCylinder(axis, radius, y_hi - y_lo).Shape()
    common = BRepAlgoAPI_Common(box, drum)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the arch")
    return common.Shape()


def build():
    block = arched(52, 0, 40, 0, 20, 0)
    cavity = arched(50, 2, 38, 2, 18, 2)
    cut = BRepAlgoAPI_Cut(block, cavity)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the cavity")
    box = BRepPrimAPI_MakeBox(gp_Pnt(14, 6, 9), gp_Pnt(26, 14, 18)).Shape()
    fuse = BRepAlgoAPI_Fuse(cut.Shape(), box)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the boss")
    part = fuse.Shape()
    side = face_through(part, (20, 14, 16))  # above the arch, whose crown is at z = 12
    return drafted(part, [side], 5, neutral=(0, 0, 11))
