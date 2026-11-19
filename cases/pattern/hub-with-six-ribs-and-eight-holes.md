+++
title = "A flanged hub with six ribs and eight bolt holes, two rings patterned about its axis"
tags = ["pattern", "polar", "union", "cut", "planar", "cylinder", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestCircularPattern.py::testCircularPatternCreatesConcentricCopies; src/Mod/PartDesign/PartDesignTests/TestPolarPattern.py::testZAxisPolarPattern"

[volume]
value = 87620.676202559922
exact = "π·50²·8 + π·15²·32 + 6·20·S − π·8²·40 − 8·π·3²·8, S = 40·4 − (2√221 + 15² asin(2/15)) = 160 − 2√221 − 225 asin(2/15): the flange, the hub above it, six ribs (each its plan outside the hub, S, 20 tall), less the bore and the bolt holes"
tex = '\pi\cdot 50^2\cdot 8 + \pi\cdot 15^2\cdot 32 + 6\cdot 20\,S - \pi\cdot 8^2\cdot 40 - 8\pi\cdot 3^2\cdot 8,\quad S = 160 - 2\sqrt{221} - 225\arcsin\tfrac{2}{15}'

[area]
value = 29630.353792534296
exact = "7512π − 3600 asin(2/15) + 10080 − 240√221: the flange's faces (bottom 2364π, rim 800π, top 2203π less the ribs' feet 6S), the hub's side 960π less the ribs' joints 6·20·2·15 asin(2/15), its top annulus 161π, the bore 640π, the holes 384π, and per rib its top S, two sides 20(40 − √221) each and its end 4·20"
tex = '7512\pi - 3600\arcsin\tfrac{2}{15} + 10080 - 240\sqrt{221}'
+++

A flanged hub with six radial ribs and a bolt circle of eight holes: two rings of features, each
one feature patterned about the z axis.

- **The flange:** a disk of radius 50 on the z axis, from z = 0 to z = 8.
- **The hub:** a cylinder of radius 15 on the z axis, from z = 8 up to z = 40, joined to the
  flange.
- **The ribs:** one rib is the box 0 ≤ x ≤ 40, −2 ≤ y ≤ 2, 8 ≤ z ≤ 28, standing on the flange
  and running out from inside the hub; the part of it outside the hub is a plate 4 thick
  sticking out of the hub's side from ρ = 15 to its flat end at x = 40, 20 tall. It is patterned
  about the z axis every 60°: six ribs, the first along +x. All are joined to the hub and flange.
- **The bore:** a hole of radius 8 on the z axis, right through, from z = 0 to z = 40.
- **The bolt holes:** one hole of radius 3 on the vertical axis through (45, 0), right through
  the flange from z = 0 to z = 8, patterned about the z axis every 45°: eight holes, the first on
  the +x axis, so two of them (at 0° and 180°) lie just beyond the ends of ribs.

No bolt hole touches a rib (the ribs' ends reach ρ ≈ 40.05 at their corners, the holes start at
ρ = 42), and the ribs meet each other only inside the hub.

**As a set:** with R the rib's box and H the bolt hole's cylinder {(x − 45)² + y² ≤ 3²},

$$\Bigl(\{\rho \le 50,\ 0 \le z \le 8\} \cup \{\rho \le 15,\ 0 \le z \le 40\} \cup \bigcup_{k=0}^{5} R_z(60^\circ k)\,R\Bigr) \setminus \{\rho \le 8\} \setminus \bigcup_{k=0}^{7} R_z(45^\circ k)\,H$$
