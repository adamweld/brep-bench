"""A void spun by a cube turned about its long diagonal inside a block. A block, the box from (−3,
−3, −3) to (3, 3, 3), with a closed void inside it: everything a cube passes through as it turns a
whole turn about its own long diagonal, wholly inside the block.

The cube is the box from (−1, −1, −1) to (1, 1, 1). It turns rigidly a whole turn about the line
through (−1, −1, −1) and (1, 1, 1), its long diagonal. Cut from the block everything the cube
passes through.

The cube's six edges that meet the diagonal's ends sweep two cones, and its six other edges, skew
to the diagonal, sweep a hyperboloid of one sheet between them, of waist radius √2. The void
touches nothing outside it: the result is the block with a sealed cavity, bounded by two shells."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Ax1, gp_Dir, gp_Pnt

from sweep import cut, flat_contacts_turned, turned, united

BLOCK = ((-3, -3, -3), (3, 3, 3))
CUBE = ((-1, -1, -1), (1, 1, 1))


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(*CUBE[0]), gp_Pnt(*CUBE[1])).Shape()
    diagonal = gp_Ax1(gp_Pnt(*CUBE[0]), gp_Dir(1, 1, 1))
    # Each face meets the turn square-on along the plane through the diagonal and its normal:
    # its own diagonal from the corner on the turn's axis.
    swept = turned(cube, diagonal, 360, flat_contacts_turned(cube, diagonal), span=360)
    # Turned a whole turn in one sweep, faces the cube's symmetry about its diagonal carries
    # onto one another sweep the same solid: united first, then cut out as one.
    return cut(BRepPrimAPI_MakeBox(gp_Pnt(*BLOCK[0]), gp_Pnt(*BLOCK[1])).Shape(), [united(swept)])
