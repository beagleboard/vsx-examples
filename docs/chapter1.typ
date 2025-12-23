#import "helpers.typ" as bh
#import "colors.typ" as bc

// Steps counter. Should start at 1.
#let step_num = counter("step_num")
#step_num.update(1)

// A boxed counter. Used by steps in chapter 1
#let step_type_1 = context bh.beagle_box_5(cnt: step_num)

= Your First Blinky Light

== #bh.beagle_heading(img: "images/chapter1/heading1.webp")[This part of the workshop introduces you to]
- #strong("Powering") - Learn how to power your PocketBeagle 2 single-board computer.
- #strong("Connecting") - Learn how to connect the PocketBeagle 2 to another computer for programming.
- #strong("Programming") - Make your PocketBeagle 2 blink an LED using a Python program.

== #bh.beagle_heading(img: "images/chapter1/heading2.webp")[Hardware used in this exercise]

#bh.beagle_box_1(img: "images/pocketbeagle_2_back.webp")[
  #strong("PocketBeagle 2 computer")
  - A tiny computer without a display, keyboard, or disk drive.
  - Expandable with add-on boards called capes
  - Can be powered and accessed over USB
]

#bh.beagle_box_1(img: "images/chapter1/hardware2.webp")[
  #strong("Pre-programmed microSD card")
  - A removable storage card pre-programmed for the workshop.
  - Pre-programmed by your workshop instructor
  - It provides software starting point for workshop
]

#bh.beagle_box_1(img: "images/chapter1/hardware3.webp")[
  #strong("Chromebook or Macbook laptop ")
  - A computer with screen, keyboard, USB port, etc.
  - Required to access the PocketBeagle 2 computer
  - Provides power to PocketBeagle 2 computer over USB
]

#bh.beagle_box_1(img: "images/chapter1/hardware4.webp")[
  #strong("USB-C cable")
  - It should be USB-C to USB-C / USB-C to USB-A cable
  - Required to power and access PocketBeagle 2 computer
  - USB-C connects to PocketBeagle 2, other end to laptop
]

== #bh.beagle_heading(img: "images/chapter1/heading3.webp")[Software used in this exercise]

#bh.beagle_box_3(
  grid.cell(align: center + horizon, image("images/safari_logo.svg", width: 55pt)),
  grid.cell(align: center + horizon, image("images/chrome_logo.svg", width: 50pt))
)[
  #strong("Safari or Chrome browser")
  - Runs on your Macbook or Chromebook laptop computer
  - It is required to access your PocketBeagle 2 computer
  - Also required to access the Visual Studio Code Server application
]

#bh.beagle_box_2(img: "images/chapter1/software2.webp")[
  #strong("Visual Studio Code Server")
  - Provides a way to program your PocketBeagle 2 computer
  - Accessed via Safari or Chrome browser on your laptop
  - It is installed on the Linux image and runs on PocketBeagle 2 computer
]

#bh.beagle_box_3(
  grid.cell(align: center + horizon, image("images/linux_logo.svg", height: 55pt)),
  grid.cell(align: center + horizon, image("images/debian_logo.svg", height: 55pt))
)[
  #strong("Debian Linux Minimal (non-graphical) image")
  - This is the software that your instructor flashed on the SD Card
  - The Linux image used for this exercise is: #link("https://files.beagle.cc/file/beagleboard-public-2021/images/pocketbeagle2-debian-13-iot-v6.12-arm64-2025-06-11-8gb.img.xz", "pocketbeagle2-debian-13-iot-v6.12-arm64-2025-06-11-8gb.img.xz")
]

// Change margins from 2nd chapter page.
#pagebreak()
#set page(margin: (x: bh.page_margin_x, y: 30pt))

#bh.grid_column(
  bh.beagle_box_4(img: "images/chapter1/step1.webp", img_height: 120pt, col1: step_type_1)[Start with fresh ingredients, nothing plugged in.],
  bh.beagle_box_4(img: "images/chapter1/step2.webp", img_height: 120pt, col1: step_type_1)[Insert your pre-flashed Micro SD Card provided by your instructor into PocketBeagle 2 MicroSD slot.],
)

#block(width: 100%, stroke: bc.dark_orange.lighten(60%) + 1pt, radius: 6pt, inset: 8pt)[
  #grid(
    columns: (auto, auto, auto),
    row-gutter: 4pt,
    column-gutter: 4pt,
    grid.cell(colspan: 3, align: center + horizon)[#image("images/chapter1/step3.webp")],
    grid.cell(align: horizon, step_type_1),
    grid.cell(align: horizon)[#text(size: 8pt)[Plug in the USB-C on PocketBeagle 2 and USB-C/USB-A to your Chromebook or Macbook laptop]],
    grid.cell(align: center)[#text(bc.dark_orange, size: 8pt)[NOTE: USB-C to USB-A can also be used if your Chromebook doesn’t have a USB-C port]]
  )
]

#bh.grid_column(
  bh.beagle_box_4(img: "images/chapter1/step4.webp", img_height: 100pt, col1: step_type_1)[USB connection provides powers to your PocketBeagle 2, you should see a Red LED light marked ‘P’ lit up.],
  bh.beagle_box_4(img: "images/chapter1/step5.webp", img_height: 100pt, col1: step_type_1)[Wait for 2 minutes while your PocketBeagle 2 computer boots up to start a network connection.],
)

#bh.beagle_box_4(img: "images/chapter1/step6.webp", col1: step_type_1)[To access PocketBeagle 2 computer using your MacBook computer open Safari or Chrome browser on your Macbook or Chromebook computer and in search bar type address 192.168.7.2 and then click on VSCode-examples.html]

#pagebreak()

#bh.grid_column(
  bh.beagle_box_4(img: "images/chapter1/step7.webp", img_height: 150pt, col1: step_type_1)[If a warning like this is shown on your screen then click on Advanced button to open up an option to access 192.168.7.2],
  bh.beagle_box_4(img: "images/chapter1/step8.webp", img_height: 150pt, col1: step_type_1)[Click on Proceed to 192.168.7.2 (unsafe) button to access Visual Studio Code Server, this is a one time process only.],
)

#block(width: 100%, fill: bc.dark_orange.lighten(93%), inset: 7pt, radius: 7pt)[
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

#block(width: 100%, stroke: bc.dark_orange.lighten(60%) + 1pt, radius: 6pt, inset: 8pt)[
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
      block(width: 100%, fill: bc.dark_orange.lighten(93%), inset: 8pt, radius: 7pt)[
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
      block(fill: rgb("#ffed00"), inset: (x: 4pt, y: 2.5pt), radius: (left: 4pt, right: 0pt), outset: (right: 4pt))[#bh.beagle_heading(img: "images/lightbulb.svg")[Get to know your tool:]],
      block(fill: bc.blue, inset: 5pt, radius: 4pt)[#text(white)[Visual Studio Code Server]],
    )
  ),
)

#pagebreak()

#grid(
  columns: (70%, 30%),
  column-gutter: 8pt,
  bh.beagle_box_4(img: "images/chapter1/step11.webp", img_height: 180pt, col1: step_type_1)[
    Navigate to the blinky example present at location shown below: #strong[vsx-examples/PocketBeagle-2/blinky/python/main.py]
  ],
  bh.beagle_box_4(img: "images/chapter1/step12.webp", img_height: 180pt, col1: step_type_1)[Click on RUN button to run the Blinky example code],
)

#block(width: 100%, stroke: bc.dark_orange.lighten(60%) + 1pt, radius: 6pt, inset: 8pt)[
    #place()[#image("images/chapter1/step13.webp")]
    #v(140pt)
    #grid(
      columns: (auto, 260pt),
      column-gutter: 4pt,
      grid.cell(align: horizon, step_type_1),
      grid.cell(align: horizon, text(size: 8pt)[Observe the text ON & OFF printed in the terminal and the green Light marked with text 4 blinking in sync with the text!])
    )
]

#bh.beagle_box_4(img: "images/chapter1/step14.webp", col1: step_type_1)[
  On line 5 change the path to blink any other light of your choice, to test let’s replace usr4 at the end with usr1 to blink light 1 Updated code on line 5 for Blinking the light 1 should look like this: LED = Path("/sys/class/leds/beaglebone:green:usr1") On line 6 update number_of_blinks from 5 to 10 as well, hit RUN button and observe Light 1 blinking for 10 times.
]
