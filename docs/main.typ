#import "helpers.typ" as bh

// Basic color define
#let box_border_color = rgb("#8c6a39")
#let beagle_background_color = rgb("#f26935")
#let beagle_text_color = rgb("#dd5a28")
#let box_border_color3 = beagle_text_color.lighten(60%)

#let page_margin_x = 15pt

// Chapter counter
#let chapter_num = counter("chapter_num")

// Steps counter. Should start at 1. Reusable.
#let step_num = counter("step_num")
#step_num.step()

// Basic document Properties
#set document(title: "PocketBeagle® 2 Coding Workshop")

// Use Gotham Rounded Font. The weird weight is required to use the Book variant as default.
#set text(font: "Gotham Rounded", weight: 330, size: 10pt)

// Set different color for bold and link text
#show strong: set text(beagle_text_color)
#show link: set text(beagle_text_color)
#show title: set text(size: 22pt, weight: 340)

/* Function to get header for current page. Currently 2 types of headers.
 * 1. Pages where new chapter starts.
 * 2. Pages part of an already started chapter.
 */
#let beagle_header() = context {
  if bh.is_chapter_start() {
    align(center + horizon)[
      #block(radius: (top: 0pt, bottom: 27pt), stroke:  (top: 0pt, rest: 3pt + rgb("#5a5b5d")), height: 100%, width: 100%, fill: rgb("#d9d9d9"))[
        #title() 
        #text(size: 12pt, weight: 340)[Chapter #chapter_num.display(). #bh.next_heading()]
      ]
    ]
  } else [
    #align(center + horizon)[
      #text(size: 10pt, weight: 340)[Chapter #chapter_num.display(). #bh.last_heading(), Continued...] 
    ]
  ]
}

/* Function to get footer for current page. Currently footer is only rendered at the first page of any chapter.
 */
#let beagle_footer() = context {
  if bh.is_chapter_start() [
    #block(fill: beagle_background_color, width: 100%, height: 100%, outset: (x: page_margin_x, y: 0pt))[
      #align(center + horizon)[
        #text(white, weight: "bold", size: 14pt)["The BeagleBoard.org Foundation is a 501(c)(3) non-profit corporation existing to provide education in and collaboration around the design and use of open-source software and hardware in embedded computing."]
      ]
    ]
  ] else if bh.is_chapter_end() [
    #align(center)[
      #grid(
        columns: (auto, auto),
        column-gutter: 4pt,
        grid.cell(
          align: center + horizon, 
          block(fill: rgb("#5a5b5d"), radius: 4pt, inset: 5pt)[
            #grid(
              columns: (auto, auto),
              column-gutter: 2pt,
              image("images/star.svg", height: 10pt),
              text(white, size: 9pt, weight: 350)[Congratulations!]
            )
          ]
        ),
        grid.cell(
          align: center + horizon, 
          text(rgb("#5a5b5d"), size: 9pt, weight: 340)[You have successfully completed Chapter #chapter_num.display(). #bh.last_heading() Lab with PocketBeagle 2 :)]
        ),
      )
    ]
  ] else [
    #none
  ]
}

// Basic doc setup
#set page(
  margin: (x: page_margin_x),
  header-ascent: 10%,
  header: beagle_header(),
  footer-descent: 10%,
  footer: beagle_footer(),
)

// Custom heading rendering with background
#show heading.where(level: 2): it => block(fill: beagle_background_color, radius: 4pt, inset: (x: 6pt, y: 3pt))[
  #set text(white)
  #it.body
]

// Do not render heading level 1. They signify chapters and will be part of header.
#show heading.where(level: 1): it => {
  chapter_num.step()
  set page(margin: (x: page_margin_x, y: auto))
}

// Common base for box type 1,2,3
#let beagle_box_123_base(stroke: color, cols: array, ..body) = {
  block(radius: 7pt, stroke: stroke + 1pt, width: 100%, inset: 6pt)[
    #grid(columns: cols, align: horizon, ..body)
  ]
}

// Common base for box type 1, 2 since they only have a single image.
#let beagle_box_12_base(stroke: color, img: str, ..body) = {
  beagle_box_123_base(
    stroke: stroke, 
    cols: (25%, auto), 
    grid.cell(align: center + horizon, image(img, height: 55pt)),
    ..body)
}

#let beagle_box_1(img: str, ..body) = beagle_box_12_base(img: img, stroke: box_border_color, ..body)

#let beagle_box_2(img: str, ..body) = beagle_box_12_base(img: img, stroke: rgb("#23b0e6"), ..body)

#let beagle_box_3(..body) = beagle_box_123_base(stroke: rgb("#23b0e6"), cols: (12.5%, 12.5%, auto), ..body)

// A boxed counter. Used by steps in chapter 1
#let step_type_1 = context {
  step_num.step()
  block(fill: beagle_text_color, inset: 4pt, radius: 4pt)[#text(white, size: 18pt, weight: "bold")[#step_num.display()]]
}

#let beagle_box_4(img: str, img_height: auto, ..body) = {
  block(width: 100%, stroke: box_border_color3 + 1pt, radius: 6pt, inset: 8pt)[
    #grid(
      columns: (auto, auto),
      row-gutter: 4pt,
      column-gutter: 4pt,
      grid.cell(colspan: 2, align: center + horizon)[#image(img, height: img_height)],
      grid.cell(align: horizon, step_type_1),
      grid.cell(align: horizon, text(size: 8pt, ..body))
    )
  ]
}

= Your First Blinky Light

== #bh.beagle_heading(img: "images/chapter1/heading1.webp")[This part of the workshop introduces you to]
- #strong("Powering") - Learn how to power your PocketBeagle 2 single-board computer.
- #strong("Connecting") - Learn how to connect the PocketBeagle 2 to another computer for programming.
- #strong("Programming") - Make your PocketBeagle 2 blink an LED using a Python program.

== #bh.beagle_heading(img: "images/chapter1/heading2.webp")[Hardware used in this exercise]
#beagle_box_1(img: "images/pocketbeagle_2_back.webp")[
  #strong("PocketBeagle 2 computer")
  - A tiny computer without a display, keyboard, or disk drive.
  - Expandable with add-on boards called capes
  - Can be powered and accessed over USB
]

#beagle_box_1(img: "images/chapter1/hardware2.webp")[
  #strong("Pre-programmed microSD card")
  - A removable storage card pre-programmed for the workshop.
  - Pre-programmed by your workshop instructor
  - It provides software starting point for workshop
]

#beagle_box_1(img: "images/chapter1/hardware3.webp")[
  #strong("Chromebook or Macbook laptop ")
  - A computer with screen, keyboard, USB port, etc.
  - Required to access the PocketBeagle 2 computer
  - Provides power to PocketBeagle 2 computer over USB
]

#beagle_box_1(img: "images/chapter1/hardware4.webp")[
  #strong("USB-C cable")
  - It should be USB-C to USB-C / USB-C to USB-A cable
  - Required to power and access PocketBeagle 2 computer
  - USB-C connects to PocketBeagle 2, other end to laptop
]

== #bh.beagle_heading(img: "images/chapter1/heading3.webp")[Software used in this exercise]

#beagle_box_3(
  grid.cell(align: center + horizon, image("images/safari_logo.svg", width: 55pt)),
  grid.cell(align: center + horizon, image("images/chrome_logo.svg", width: 50pt))
)[
  #strong("Safari or Chrome browser")
  - Runs on your Macbook or Chromebook laptop computer
  - It is required to access your PocketBeagle 2 computer
  - Also required to access the Visual Studio Code Server application
]

#beagle_box_2(img: "images/chapter1/software2.webp")[
  #strong("Visual Studio Code Server")
  - Provides a way to program your PocketBeagle 2 computer
  - Accessed via Safari or Chrome browser on your laptop
  - It is installed on the Linux image and runs on PocketBeagle 2 computer
]

#beagle_box_3(
  grid.cell(align: center + horizon, image("images/linux_logo.svg", height: 55pt)),
  grid.cell(align: center + horizon, image("images/debian_logo.svg", height: 55pt))
)[
  #strong("Debian Linux Minimal (non-graphical) image")
  - This is the software that your instructor flashed on the SD Card
  - The Linux image used for this exercise is: #link("https://files.beagle.cc/file/beagleboard-public-2021/images/pocketbeagle2-debian-13-iot-v6.12-arm64-2025-06-11-8gb.img.xz", "pocketbeagle2-debian-13-iot-v6.12-arm64-2025-06-11-8gb.img.xz")
]

// Change margins from 2nd chapter page.
#pagebreak()
#set page(margin: (x: page_margin_x, y: 30pt))

#bh.grid_column(
  beagle_box_4(img: "images/chapter1/step1.webp", img_height: 120pt)[Start with fresh ingredients, nothing plugged in.],
  beagle_box_4(img: "images/chapter1/step2.webp", img_height: 120pt)[Insert your pre-flashed Micro SD Card provided by your instructor into PocketBeagle 2 MicroSD slot.],
)

#block(width: 100%, stroke: box_border_color3 + 1pt, radius: 6pt, inset: 8pt)[
  #grid(
    columns: (auto, auto, auto),
    row-gutter: 4pt,
    column-gutter: 4pt,
    grid.cell(colspan: 3, align: center + horizon)[#image("images/chapter1/step3.webp")],
    grid.cell(align: horizon, step_type_1),
    grid.cell(align: horizon)[#text(size: 8pt)[Plug in the USB-C on PocketBeagle 2 and USB-C/USB-A to your Chromebook or Macbook laptop]],
    grid.cell(align: center)[#text(beagle_text_color, size: 8pt)[NOTE: USB-C to USB-A can also be used if your Chromebook doesn’t have a USB-C port]]
  )
]

#bh.grid_column(
  beagle_box_4(img: "images/chapter1/step4.webp", img_height: 100pt)[USB connection provides powers to your PocketBeagle 2, you should see a Red LED light marked ‘P’ lit up.],
  beagle_box_4(img: "images/chapter1/step5.webp", img_height: 100pt)[Wait for 2 minutes while your PocketBeagle 2 computer boots up to start a network connection.],
)

#beagle_box_4(img: "images/chapter1/step6.webp")[To access PocketBeagle 2 computer using your MacBook computer open Safari or Chrome browser on your Macbook or Chromebook computer and in search bar type address 192.168.7.2 and then click on VSCode-examples.html]

#pagebreak()

#bh.grid_column(
  beagle_box_4(img: "images/chapter1/step7.webp", img_height: 150pt)[If a warning like this is shown on your screen then click on Advanced button to open up an option to access 192.168.7.2],
  beagle_box_4(img: "images/chapter1/step8.webp", img_height: 150pt)[Click on Proceed to 192.168.7.2 (unsafe) button to access Visual Studio Code Server, this is a one time process only.],
)

#block(width: 100%, fill: beagle_text_color.lighten(93%), inset: 7pt, radius: 7pt)[
  #grid(
    columns: (auto, auto),
    row-gutter: 7pt,
    grid.cell(colspan: 2, text(weight: "medium")[Why Does the 'Your connection is not private' Warning Show Up?]),
        
    text(size: 7pt)[Imagine you’re sending a secret note to a friend, and your browser (like Chrome or Safari) wants to make sure no one else can read it. To do this, it checks for a special "lock" (called an SSL certificate) on the website. If lock is not present it will show that warning!],
        
    grid.cell(rowspan: 4, image("images/chapter1/note1.webp", width: 60pt)),
        
    text(weight: "medium")[But wait!],
        
    text(size: 7pt)[Your PocketBeagle is like a tiny computer right next to you—it’s not a website on the internet. Since it’s brand new and just for your class, it doesn’t have that "lock" yet. The warning is like your browser saying, "Hmm, I don’t recognize this friend!"],
        
    text(weight: "medium")[Is it Safe?],
        
    grid.cell(
      colspan: 2,
      bh.beagle_heading(img: "images/checkmark.svg", img_height: 8pt)[#text(size: 7pt)[Yes! You’re connecting directly to your own PocketBeagle 2, not the internet. It’s like talking to a classmate in the same room—no strangers involved.]]
    ),
        
    grid.cell(
      colspan: 2,
      text(weight: "medium", size: 9pt)[Remember: Only do this for your PocketBeagle in class. On other websites, always stay safe and listen to the warnings!]
    ),
  )
]

#block(width: 100%, stroke: box_border_color3 + 1pt, radius: 6pt, inset: 8pt)[
  #grid(
    columns: (auto, auto, 68%),
    rows: (60pt, auto),
    row-gutter: 4pt,
    column-gutter: 8pt,
    grid.cell(align: horizon, step_type_1),
    grid.cell(align: horizon, text(size: 8pt)[Click on Yes, I trust the authors button to start exploring examples.]),
    grid.cell(rowspan: 2, image("images/chapter1/step9.webp")),
    grid.cell(
      colspan: 2,
      align: bottom,
      block(width: 100%, fill: beagle_text_color.lighten(93%), inset: 8pt, radius: 7pt)[
        #text(weight: "medium")[Why are we doing this?]
        
        #text(size: 7pt)[This just verifies the files are from your teacher/classroom device, you should trust it to safely start coding!]
      ]
    )
  )
]

#grid(
  columns: (1fr, 1fr),
  row-gutter: 10pt,
  column-gutter: 8pt,
  grid.cell(align: center, colspan: 2, image("images/chapter1/step10.webp", height: 250pt)),
  grid.cell(
    align: center + horizon,
    colspan: 2,
    grid(
      columns: (auto, auto, auto),
      column-gutter: (4pt, 0pt),
      grid.cell(align: horizon, step_type_1),
      block(fill: rgb("#ffed00"), inset: 4pt, radius: (left: 4pt, right: 0pt), outset: (right: 4pt))[Get to know your tool:],
      block(fill: rgb("#23b0e6"), inset: 4pt, radius: 4pt)[Visual Studio Code Server],
    )
  ),
)

#pagebreak()

#grid(
  columns: (70%, 30%),
  column-gutter: 8pt,
  beagle_box_4(img: "images/chapter1/step11.webp", img_height: 180pt)[
    Navigate to the blinky example present at location shown below: #strong[vsx-examples/PocketBeagle-2/blinky/python/main.py]
  ],
  beagle_box_4(img: "images/chapter1/step12.webp", img_height: 180pt)[Click on RUN button to run the Blinky example code],
)

#block(width: 100%, stroke: box_border_color3 + 1pt, radius: 6pt, inset: 8pt)[
    #place()[#image("images/chapter1/step13.webp")]
    #v(140pt)
    #grid(
      columns: (auto, 260pt),
      column-gutter: 4pt,
      grid.cell(align: horizon, step_type_1),
      grid.cell(align: horizon, text(size: 8pt)[Observe the text ON & OFF printed in the terminal and the green Light marked with text 4 blinking in sync with the text!])
    )
]

#beagle_box_4(img: "images/chapter1/step14.webp")[
  On line 5 change the path to blink any other light of your choice, to test let’s replace usr4 at the end with usr1 to blink light 1 Updated code on line 5 for Blinking the light 1 should look like this: LED = Path("/sys/class/leds/beaglebone:green:usr1") On line 6 update number_of_blinks from 5 to 10 as well, hit RUN button and observe Light 1 blinking for 10 times.
]
