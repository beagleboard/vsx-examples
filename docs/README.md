# Creating PDF docs using SILE Typesetter

[The SILE Typesetter](https://sile-typesetter.org/) is a modern rewrite of TeX. It is not fully compatible, but it is bundled nicely
for simple PDF generation.

In addition to Tex-style input, you can also use XML. There is a one-to-one correspondance between the styles. I've opted to use XML
for my examples as I'm more familiar with it.

## Getting started

To get familiar with the SILE Typesetter, consider reading [The SILE Book](https://sile-typesetter.org/manual/sile-0.15.9.pdf). There
are some big holes in the documentation, but I hope to fill you in.

### Installation

On Ubuntu 24.04, I've set it up roughly via:

```sh
sudo apt-get update && apt-get install -y software-properties-common unzip curl git
sudo add-apt-repository ppa:sile-typesetter/sile
DEBIAN_FRONTEND=noninteractive apt-get install -y sile luarocks
curl https://openbeagle.org/jkridner/fonts/-/raw/main/gotham-rounded.zip?inline=false -o /tmp/gotham-rounded.zip
unzip /tmp/gotham-rounded.zip -d /usr/share/fonts/
sudo fc-cache -f
luarocks install --tree lua_modules markdown.sile
luarocks install --tree lua_modules highlighter.sile
```

NOTE: I'm using a couple of 3rd party plugins at the moment to be able to render Markdown and perform syntax highlighting easily. I am choosing to
simply install them in the directory where I plan to run `sile`.

### Hello, SILE!

Let's give it a try.

```sh
cat <<EOF >hello.xml
<document>
Hello, SILE!
</document>
EOF
sile hello.xml
xdg-open hello.pdf
```

So, the minimum needed is the `<document>` command. The XML tags are run as "commands" in SILE terminology. These end up translating pretty
directly to calls in the Lua code.

As you can see, a PDF is generated

## Handouts class

I have stolen a bunch from existing document classes to create the `handouts` class. You'll find this in [classes/handouts.lua](classes/handouts.lua).

### Hello, Handouts!

```sh
cat <<EOF >hello-handouts.xml
sile hello-handouts.xml
<document class="handouts" papersize="letter">
<place top="30%ph" bottom="35%ph" left="10%pw" right="90%pw">
<center>
Hello, Handouts!
</center>
</place>
<eject />

<place top="30%ph" bottom="35%ph" left="10%pw" right="90%pw">
<center>
Hello, Handouts page 2!
</center>
</place>
</document>
xdg-open hello-handouts.pdf
```

This is mostly the same, but by setting `class="handouts"` in the `document` tag, we now have access to a few new commands. You will see
`place`, `center` and `eject` tags/commands used to help position some content on the page.

In the `place` command, you will see "top", "bottom", "left", and "right" all defined. These are mandatory for this command. Further, you should
note that you can specify their settings using "_XXX_%pw" or "_XXX_%ph", this is a percentage of the page width or page height, respectively. There
are other options you can use with SILE, but this one seems pretty simple to decide where you want something on the page by setting it directly.

The `center` command simply center-aligns the text horizontally in the frame and leaves it "ragged".

The `eject` command is used to start a new page.

NOTE: I have no idea why I need a blank line after the `eject`, but it seems to be necessary.

### Commands in handouts.lua

If you see a command name used in other classes in SILE, it is most likely the same implementation copied. Only `place` and `highlight` are really new.

There are also several commands imported from other libraries, including the ones installed earlier and ones shipped with SILE.

I believe these are the most useful commands are:

* place(top,bottom,left,right) - put content on the page in a specified frame location
* highlight(src,[language]) - include a source file with syntax highlighting
* include(src) - include a markdown file
* center - center align text
* eject - start a new page
* svg(src,[width]) - include an SVG formatted image file (see SILE documentation)
* image - include an image file (see SILE documentation)
* lorem(words) - add a specified number of words to the page to help with testing
* font - see SILE documentation
* noindent - do not indent to start the paragraph
* smallskip, medskip, bigskip - add some extra space between paragraphs

