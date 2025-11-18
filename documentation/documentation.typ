#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge
#import fletcher.shapes: cylinder, hexagon, house, pill, octagon

#set text(
  lang: "pt",
  region: "br",
  font: "IBM Plex Sans",
  weight: "regular",
  size: 11pt
)

\

= SUMMARY

\

This is the documentation for Rustart, you can change this and run it with 'mask docs', see README.md for more details.

\


#line(length: 100%, stroke: 0.1pt)

\

== SOFTWARE ARCHITECTURE
\
#let architecture_diagram() = diagram(
  node-fill: white,
  node-stroke: luma(80%),

  node((0,0), [Datastar Frontend Framework], name: <frontend_html>, shape: pill, inset: 10pt),
  node((2,0), [Common API], name: <frontend_json>, shape: pill, inset: 10pt),
  node((1,2), [Rust Backend], name: <back>, shape: pill, inset: 10pt),
  node((1,4), [PostgreSQL], name: <postgres>, shape: cylinder),
  node((1,4), [PostgreSQL], name: <postgres>, shape: cylinder),

  edge(<frontend_json>, <back>, "<->", label: "JSON", label-side: center),
  edge(<frontend_html>, <back>, "<->", label: "HTML", label-side: center),

  edge(<postgres>, <back>, "<->", label: "SQLx (ORM)", label-sep: 12pt, label-side: left),
  edge(<postgres>, <back>, "<->", label: "SQLxMQ (Queue)", label-side:right, label-sep: 12pt),

)

#architecture_diagram()

== Parts
\

We start with a docker compose
The pattern used is inspired in Domain Driven Design (DDD) and Clean Architecture.
There is an application layer for business logic, domain for pure entities that closely relate to the data modeled and impure entities for
any differentiation in that due to an implementation limitation.
Infrastructure is for the relationship to outside services such as repositories and controllers.
