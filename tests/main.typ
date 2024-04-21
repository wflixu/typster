#set page(
  paper: "a4",
  margin: (x: 1.8cm, y: 1.5cm),
)
#set text(
  font: "New Computer Modern",
  size: 16pt
)

#set par(
  justify: true,
  leading: 0.52em,
)


= Introduction

In this report, we will explore the
various factors that influence fluid
dynamics in glaciers and how they
contribute to the formation and
behaviour of these natural structures.

+ The climate
  - Temperature
  - Precipitation
+ The topography
+ The geology

#image("./app-icon.png")


The flow rate of a glacier is
defined by the following equation:

$ Q = rho A v + C $



= Introduction

In this report, we will explore the
various factors that influence fluid
dynamics in glaciers and how they
contribute to the formation and
behaviour of these natural structures.
 
= picture
 #figure(
  image("app-icon.png", width: 70%),
  caption: [
    _Glaciers_ form an important part
    of the earth's climate system.
  ],
)


= Methods
We follow the glacier melting models



The equation $Q = rho A v + C$
defines the glacial flow rate.

Total displaced soil by glacial flow:

$ 7.32 beta +
  sum_(i=0)^nabla
    (Q_i (a_i - epsilon)) / 2 $


$ v := vec(x_1, x_2, x_3) $


#par(justify: true)[
  = Background
  In the case of glaciers, fluid
  dynamics principles can be used
  to understand how the movement
  and behaviour of the ice is
  influenced by factors such as
  temperature, pressure, and the
  presence of other fluids (such as
  water).
]


== Background

#lorem(100)


= bib

#lorem(40)  
@smith2020

#bibliography("works.bib", full: true)
