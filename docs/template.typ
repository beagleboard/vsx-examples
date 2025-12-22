#import "helpers.typ" as bh
#import "colors.typ" as bc

/// Chapter helpers

// Check if a new chapter starts at the current page.
#let is_chapter_start() = query(heading.where(level: 1)).map(h => h.location().page()).contains(here().page())
// Helper to know if a new chapter starts at the next page.
// Also handles last page since that is also considered chapter end.
#let is_chapter_end() = query(heading.where(level: 1)).map(h => h.location().page()).contains(here().page() + 1) or here().page() == counter(page).final().first()
// Return the next heading
#let next_heading() = query(heading.where(level: 1).after(here())).first().body
// Return the last heading
#let last_heading() = query(heading.where(level: 1).before(here())).last().body

#let conf(doc) = {
  let page_margin_x = 15pt
  
  // Chapter counter
  let chapter_num = counter("chapter_num")
  
  // Use Gotham Rounded Font. The weird weight is required to use the Book variant as default.
  set text(font: "Gotham Rounded", weight: 330, size: 10pt)
  
  // Set different color for bold and link text
  show strong: set text(bc.dark_orange)
  show link: set text(bc.dark_orange)
  show title: set text(size: 22pt, weight: 340)
  
  /* Function to get header for current page. Currently 2 types of headers.
   * 1. Pages where new chapter starts.
   * 2. Pages part of an already started chapter.
   */
  let beagle_header() = context {
    if is_chapter_start() {
      align(center + horizon)[
        #block(radius: (top: 0pt, bottom: 27pt), stroke:  (top: 0pt, rest: 3pt + rgb("#5a5b5d")), height: 100%, width: 100%, fill: rgb("#d9d9d9"))[
          #title() 
          #text(size: 12pt, weight: 340)[Chapter #chapter_num.display(). #next_heading()]
        ]
      ]
    } else [
      #align(center + horizon)[
        #text(size: 10pt, weight: 340)[Chapter #chapter_num.display(). #last_heading(), Continued...] 
      ]
    ]
  }
  
  /* Function to get footer for current page. Currently footer is only rendered at the first page of any chapter.
   */
  let beagle_footer() = context {
    if is_chapter_start() [
      #block(fill: bc.tongue_orange, width: 100%, height: 100%, outset: (x: page_margin_x, y: 0pt))[
        #align(center + horizon)[
          #text(white, weight: "bold", size: 14pt)["The BeagleBoard.org Foundation is a 501(c)(3) non-profit corporation existing to provide education in and collaboration around the design and use of open-source software and hardware in embedded computing."]
        ]
      ]
    ] else if is_chapter_end() [
      #align(center)[
        #grid(
          columns: (auto, auto),
          column-gutter: 4pt,
          grid.cell(
            align: center + horizon, 
            block(fill: bc.dark_grey, radius: 4pt, inset: 5pt)[#bh.beagle_heading(img: "images/star.svg")[#text(white, size: 9pt, weight: 350)[Congratulations!]]]
          ),
          grid.cell(
            align: center + horizon, 
            text(bc.dark_grey, size: 9pt, weight: 340)[You have successfully completed Chapter #chapter_num.display(). #last_heading() Lab with PocketBeagle 2 :)]
          ),
        )
      ]
    ] else [
      #none
    ]
  }
  
  // Basic doc setup
  set page(
    margin: (x: page_margin_x),
    header-ascent: 10%,
    header: beagle_header(),
    footer-descent: 10%,
    footer: beagle_footer(),
  )
  
  // Custom heading rendering with background
  show heading.where(level: 2): it => block(fill: bc.tongue_orange, radius: 4pt, inset: (x: 6pt, y: 3pt))[
    #set text(white)
    #it.body
  ]
  
  // Do not render heading level 1. They signify chapters and will be part of header.
  show heading.where(level: 1): it => {
    chapter_num.step()
    set page(margin: (x: page_margin_x, y: auto))
  }

  doc
}
