#import "colors.typ" as bc
#import "helpers.typ" as bh

// Steps counter. Should start at 1.
#let step_num = counter("step_num")
#step_num.update(1)

// A boxed counter. Used by steps in chapter 1
#let step_box = context bh.beagle_box_5(cnt: step_num)

= Introduction to TechLab Cape

== #bh.beagle_heading(img: "images/chapter1/heading1.webp")[This part of the workshop introduces you to]
- #strong("TechLab Cape") - Learn how to connect and use the TechLab Cape with the PocketBeagle 2 computer.
- #strong("Connecting") - You will learn how to connect TechLab Cape & PocketBeagle 2 to another computer.
- #strong("Programming") - Use the PocketBeagle 2 + TechLab cape setup to interact with buttons, LEDs, and more.

== #bh.beagle_heading(img: "images/chapter1/heading2.webp")[Additional hardware used in this exercise]

#bh.beagle_box_1(img: "images/chapter2/hardware1.webp")[
  === TechLab Cape
  - Add-on board for your PocketBeagle 2 to expand its capabilities.
  - It’s perfect for learning and experimenting with PocketBeagle 2.
  - It has Buttons, LEDs, a USB port, a Buzzer, an Accelerometer etc
]

#text(bc.dark_orange, weight: 340)[Note: You should go through Chapter 1 to know all the other required hardware for this chapter. ]


== #bh.beagle_heading(img: "images/chapter2/heading3.svg")[Prepare your hardware]

#bh.grid_column(
  bh.beagle_box_4(img: "images/chapter1/step1.webp", img_height: 110pt, col1: step_box)[Make sure you have all the hardware from #strong[Chapter 1. Your First Blinky Light]],
  bh.beagle_box_4(img: "images/chapter1/step2.webp", img_height: 110pt, col1: step_box)[Insert your pre-flashed Micro SD Card provided by your instructor into PocketBeagle 2 MicroSD slot.],
  bh.beagle_box_4(img: "images/chapter2/step3.webp", img_height: 110pt, col1: step_box)[Prepare to join your PocketBeagle 2 computer and TechLab Cape.],
  bh.beagle_box_4(img: "images/chapter2/step4.webp", img_height: 110pt, col1: step_box)[Align the board so that the header pins align and make sure the USB is on the correct sidel],
  bh.beagle_box_4(img: "images/chapter2/step5.webp", img_height: 110pt, col1: step_box)[Once the pins are aligned and USB is on the correct side press both the boards to attach.],
  bh.beagle_box_4(img: "images/chapter2/step6.webp", img_height: 110pt, col1: step_box)[A completed setup should look like this, see the orientation of the board carefully.],
)

// Change margins from 2nd chapter page.
#pagebreak()
#set page(margin: (x: bh.page_margin_x, y: 30pt))

#block(radius: 7pt, stroke: bc.dark_orange.lighten(60%) + 1pt, width: 100%, inset: 12pt)[
  == #bh.beagle_heading(img: "images/chapter2/heading4.svg")[Know your setup]
  #align(center)[#image("images/chapter2/know_your_setup.webp", height: 270pt)]
]

#block(width: 100%, stroke: bc.dark_orange.lighten(60%) + 1pt, radius: 6pt, inset: 8pt)[
  #grid(
    columns: (auto, auto, auto),
    gutter: 4pt,
    grid.cell(colspan: 3, align: center + horizon)[#image("images/chapter2/step7.webp", height: 200pt)],
    grid.cell(align: horizon, step_box),
    grid.cell(align: horizon)[#text(size: 8pt)[To start with the exercise you have to connect your PocketBeagle 2 computer + TechLab cape setup to your MacBook computer as shown in the connection diagram shown above.]],
    grid.cell(align: center)[#text(bc.dark_orange, size: 8pt)[NOTE: USB-C to USB-A can also be used if your Chromebook doesn’t have a USB-C port]]
  )
]

#bh.grid_column(
  bh.beagle_box_4(img: "images/chapter1/step4.webp", img_height: 110pt, col1: step_box)[USB connection provides powers to your PocketBeagle 2, you should see a Red LED light marked ‘P’ lit up.],
  bh.beagle_box_4(img: "images/chapter2/step9.webp", img_height: 110pt, col1: step_box)[On successful boot-up, you should see an LED pattern on the seven-segment display of the TechLab cape.],
)

#pagebreak()

#bh.beagle_box_4(img: "images/chapter1/step6.webp", col1: step_box)[To access PocketBeagle 2 computer using your MacBook computer open Safari or Chrome browser on your Macbook or Chromebook computer and in search bar type address #strong[192.168.7.2] and then click on #strong[VSCode-examples.html]]

#block(width: 100%, stroke: bc.dark_orange.lighten(60%) + 1pt, radius: 6pt, inset: 12pt)[
  #grid(
    columns: (auto, auto),
    gutter: 4pt,
    grid.cell(align: horizon)[#text(bc.dark_orange, size: 27pt, weight: 340)[Note:]],
    grid.cell(align: horizon)[#text(bc.dark_orange, size: 9pt)[Here, it is expected that you have already gone through the Visual Studio Code setup in Chapter 1 (Steps 7, 8, and 9) where we accept some safety warnings to use Visual Studio Code a browser.]]
  )
]

#bh.grid_column(
  columns: (70%, 30%),
  bh.beagle_box_4(img: "images/chapter2/step11.webp", img_height: 180pt, col1: step_box)[
    Navigate to the button example present at location shown below: #strong[vsx-examples/PocketBeagle-2/button/python/main.py]
  ],
  bh.beagle_box_4(img: "images/chapter1/step12.webp", img_height: 180pt, col1: step_box)[Click on #strong[RUN] button to run the Button example code],
)

#bh.beagle_box_6(img: "images/chapter2/step13.webp", v_height: 174pt, col_width: 250pt, col1: step_box)[Observe the text #strong[“Right”] printed in the terminal when you press the button with marking R (see image).]

#pagebreak()

#block(width: 100%, stroke: bc.dark_orange.lighten(60%) + 1pt, radius: 6pt, inset: 12pt)[
  #grid(
    columns: (auto, auto),
    column-gutter: 4pt,
    row-gutter: 8pt,
    grid.cell[== #bh.beagle_heading(img: "images/chapter2/heading5.webp")[Know your setup]],
    grid.cell(rowspan: 2, align: bottom)[#image("images/chapter2/keys_as_numbers.webp", height: 80pt)],
    text(size: 8pt)[
      In Linux, every key on a keyboard is assigned a unique number called a key code. When you press a key, Linux doesn’t see #strong[“left”] or #strong[“right”], it sees numbers. For example, the left arrow key is 105, and the right arrow key is 106. These numbers are defined in the Linux kernel so that all programs and devices can understand key presses in a standard way. When a key is pressed or released, the system sends this number along with the action (press or release), and the software determines what to do, such as moving a cursor or scrolling a page.
    ],
    grid.cell(colspan: 2, align: center)[#text(bc.dark_orange)[Reference: https://elixir.bootlin.com/linux/v6.15.7/source/include/uapi/linux/input-event-codes.h#L180-L181]]
  )
]

#bh.beagle_box_4(img: "images/chapter2/step14.webp", col1: step_box)[Edit line 8 to change the button code to 105 (for the left button). Update line 19 to print "Left" when it is pressed. Click Run to test. If everything is set correctly, you’ll see "Left" printed in the terminal when the left button is pressed.]

#block(width: 100%, stroke: bc.dark_orange.lighten(60%) + 1pt, radius: 6pt, inset: 8pt)[
    == #bh.beagle_heading(img: "images/chapter2/heading6.webp")[Coding Challenges]
    + Turn on any LED on the PocketBeagle 2 computer using any of the buttons on the TechLab Cape.
    + Demonstrate the difference between waiting for the press vs waiting for the release of the Button
    + Detect both the Right and Left buttons simultaneously, print all three states on terminal.
]
