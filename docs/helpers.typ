/* This module is intended to store helper functions to make it easier to create a document with a 
 * cohesive theme.
 */

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
#let grid_column(..body) = grid(columns: (1fr, 1fr), column-gutter: 8pt, ..body)
