/* This module is intended to store helper functions to make it easier to create a document with a 
 * cohesive theme.
 */

#import "colors.typ" as bc

/// Bordered boxes with various styles

// Box that lays out elements in a grid
#let beagle_box_123_base(stroke: color, cols: array, ..body) = {
  block(radius: 7pt, stroke: stroke + 1pt, width: 100%, inset: 6pt)[
    #grid(columns: cols, align: horizon, ..body)
  ]
}

// Box with 2 columns: 1st for image and 2nd for content
#let beagle_box_12_base(stroke: color, img: str, ..body) = {
  beagle_box_123_base(
    stroke: stroke, 
    cols: (25%, auto), 
    grid.cell(align: center + horizon, image(img, height: 55pt)),
    ..body)
}

// 2 column box with a hair_dark_brown border
#let beagle_box_1(img: str, ..body) = beagle_box_12_base(img: img, stroke: bc.hair_dark_brown.lighten(60%), ..body)

// 2 column box with blue border
#let beagle_box_2(img: str, ..body) = beagle_box_12_base(img: img, stroke: bc.blue.lighten(60%), ..body)

// 3 column box with blue border
#let beagle_box_3(..body) = beagle_box_123_base(stroke: bc.blue.lighten(60%), cols: (12.5%, 12.5%, auto), ..body)

// 2 column box with 1st row of image and 2nd row 2 content columns.
#let beagle_box_4(img: str, img_height: auto, col1: content, ..body) = {
  set text(size: 8pt)
  block(width: 100%, stroke: bc.dark_orange.lighten(60%) + 1pt, radius: 6pt, inset: 8pt)[
    #grid(
      columns: (auto, auto),
      row-gutter: 4pt,
      column-gutter: 4pt,
      grid.cell(colspan: 2, align: center + horizon)[#image(img, height: img_height)],
      grid.cell(align: horizon, col1),
      grid.cell(align: horizon, ..body)
    )
  ]
}

// An orange box that prints a counter.
#let beagle_box_5(cnt: counter) = {
  cnt.step()
  block(fill: bc.dark_orange, inset: 4pt, radius: 4pt)[#text(white, size: 18pt, weight: "bold")[#cnt.display()]]
}

// A box with floating top image
#let beagle_box_6(img: str, v_height: auto, col_width: auto, col1: content, ..body) = block(width: 100%, stroke: bc.dark_orange.lighten(60%) + 1pt, radius: 6pt, inset: 8pt)[
    #place[#image(img)]
    #v(v_height)
    #grid(
      columns: (auto, col_width),
      column-gutter: 4pt,
      grid.cell(align: horizon, col1),
      grid.cell(align: horizon, text(size: 8pt, ..body))
    )
]


/// Misc

// A simple helper to have an image followed by some content.
#let beagle_heading(img: str, img_height: 12pt, ..body) = {
  grid(
    columns: (auto, auto),
    column-gutter: 2pt,
    grid.cell(align: horizon)[#image(img, height: img_height)],
    grid.cell(align: horizon, ..body)
  )
}

// Current normal column does not maintain height properly. So using grid instead.
#let grid_column(columns: (1fr, 1fr),..body) = grid(columns: columns, gutter: 8pt, ..body)

// Bit of a hack. But need this in both template and chapters.
#let page_margin_x = 15pt
