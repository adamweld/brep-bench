"""A slab of a horn torus, both its flats moved out. A horn torus: the solid swept by the disk of radius 5 centered on (5, 0, 0) in the plane y = 0,
turned a full turn about the z axis. Its tube touches the axis at the origin, where its surface
pinches to a point. Take the slab of it between z = −3 and z = 3. Its faces are the torus, which
meets the axis at the origin, and two flat annuli between radii 1 and 9, in the planes z = −3 and
z = 3.

Then edit the solid directly: move each flat 1 outward along its normal, the upper one up to the
plane z = 4 and the lower one down to z = −4. The torus runs on to meet them, so each flat is now
the annulus between radii 2 and 8.

So the result is the horn torus between z = −4 and z = 4."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeTorus
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused, face_through
from edit import moved

RADIUS, HEIGHT, DISTANCE = 5, 3, 1


def build():
    torus = BRepPrimAPI_MakeTorus(RADIUS, RADIUS).Shape()
    drum = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -HEIGHT), gp_Dir(0, 0, 1)), 2 * RADIUS + 1,
                                    2 * HEIGHT).Shape()
    common = BRepAlgoAPI_Common(torus, drum)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common could not cut the slab from the horn torus")
    slab = common.Shape()
    top = face_through(slab, (5, 0, HEIGHT))
    bottom = face_through(slab, (5, 0, -HEIGHT))
    return moved(slab, [(top, DISTANCE), (bottom, DISTANCE)])
