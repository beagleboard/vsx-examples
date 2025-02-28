local base = require("classes.base")

local class = pl.class(base)
class._name = "handouts"

class.defaultFrameset = {
	content = {
		left = "5%pw",
		right = "95%pw",
		top = "5%ph",
		bottom = "95%ph",
	},
}
class.firstContentFrame = "content"

local skips = {
	small = "3pt plus 1pt minus 1pt",
	med = "6pt plus 2pt minus 2pt",
	big = "12pt plus 4pt minus 4pt",
}

function class:_init (options)
	base._init(self, options)
	self:loadPackage("counters")
	SILE.scratch.counters.folio = { value = 1, display = "arabic", off = true }
	self:registerHook("newpage", function ()
		self:incrementFolio()
	end)
	self:registerHook("endpage", function ()
		self:outputFolio(options and options.frame)
	end)
	self:loadPackage("color")
	self:loadPackage("highlighter")
	self:loadPackage("markdown")
	self:loadPackage("verbatim")
	self:loadPackage("image")
	self:loadPackage("svg")
	SILE.languageSupport.loadLanguage("la")
	self:loadPackage("lorem")
	self:loadPackage("counters")
	self:loadPackage("frametricks")
	SILE.scratch.counters.place = { value = 1 }
end

function class:incrementFolio (_)
	SILE.scratch.counters.folio.value = SILE.scratch.counters.folio.value + 1
end

function class:outputFolio (frame)
	local folio = self.packages.counters:formatCounter(SILE.scratch.counters.folio)
	io.stderr:write("Processing page [" .. folio .. "]\n")
end

function class:endPage ()
	return base.endPage(self)
end

function class:finish ()
	local ret = base.finish(self)
	return ret
end

class.documentation = [[
\begin{document}
...
\end{document}
]]

function class:declareSettings ()
	base.declareSettings(self)
	for k, v in pairs(skips) do
		SILE.settings:declare({
			parameter = "plain." .. k .. "skipamount",
			type = "vglue",
			default = SILE.types.node.vglue(v),
			help = "The amount of a \\" .. k .. "skip",
		})
	end
end

function class:registerCommands ()
	SILE.classes.base.registerCommands(self)

	self:registerCommand("noindent", function (_, content)
		SILE.settings:set("current.parindent", SILE.types.node.glue())
		SILE.process(content)
	end, "Do not add an indent to the start of this paragraph")

	self:registerCommand("indent", function (_, content)
		SILE.settings:set("current.parindent", SILE.settings:get("document.parindent"))
		SILE.process(content)
	end, "Do add an indent to the start of this paragraph")

	for k, _ in pairs(skips) do
		self:registerCommand(k .. "skip", function (_, _)
			SILE.typesetter:leaveHmode()
			SILE.typesetter:pushExplicitVglue(SILE.settings:get("plain." .. k .. "skipamount"))
		end, "Skip vertically by a " .. k .. " amount")
	end

	self:registerCommand("vfill", function (_, _)
		SILE.typesetter:leaveHmode()
		SILE.typesetter:pushExplicitVglue(SILE.types.node.vfillglue())
	end, "Add huge vertical glue")

	self:registerCommand("break", function (_, _)
		SILE.call("penalty", { penalty = -10000 })
	end, "Requests a frame break (if in vertical mode) or a line break (if in horizontal mode)")

	self:registerCommand("nobreak", function (_, _)
		SILE.call("penalty", { penalty = 10000 })
	end, "Inhibits a frame break (if in vertical mode) or a line break (if in horizontal mode)")

	self:registerCommand("novbreak", function (_, _)
		SILE.call("penalty", { penalty = 10000, vertical = true })
	end, "Inhibits a frame break (switching to vertical mode if needed)")

	self:registerCommand("allowbreak", function (_, _)
		SILE.call("penalty", { penalty = 0 })
	end, "Allows a page break (if in vertical mode) or a line break (if in horizontal mode)")

	self:registerCommand("goodbreak", function (_, _)
		SILE.call("penalty", { penalty = -500 })
	end, "Indicates a good potential point for a frame break (if in vertical mode) or a line break (if in horizontal mode)")

	self:registerCommand("eject", function (_, _)
		SILE.call("vfill")
		SILE.call("break")
	end, "Fills the page with stretchable vglue and then request a page break")

	self:registerCommand("supereject", function (_, _)
		SILE.call("vfill")
		SILE.call("penalty", { penalty = -20000 })
	end, "Fills the page with stretchable vglue and then requests a non-negotiable page break")

	self:registerCommand("em", function (_, content)
		local style = SILE.settings:get("font.style")
		local toggle = (style and style:lower() == "italic") and "Regular" or "Italic"
		SILE.call("font", { style = toggle }, content)
	end, "Emphasizes its contents by switching the font style to italic (or back to regular if already italic)")

	self:registerCommand("hbox", function (_, content)
		local hbox, hlist = SILE.typesetter:makeHbox(content)
		SILE.typesetter:pushHbox(hbox)
		io.stderr:write("hbox has " .. hbox .. "\n")
		return hbox
	end, "Compiles all the enclosed horizontal-mode material into a single hbox")

	self:registerCommand("vbox", function (options, content)
		local vbox
		SILE.settings:temporarily(function ()
			if options.width then
				SILE.settings:set("typesetter.breakwidth", SILE.types.length(options.width))
			end
			SILE.typesetter:pushState()
			SILE.process(content)
			SILE.typesetter:leaveHmode(1)
			vbox = SILE.pagebuilder:collateVboxes(SILE.typesetter.state.outputQueue)
			SILE.typesetter:popState()
		end)
		return vbox
	end, "Compiles all the enclosed material into a single vbox")

	self:registerCommand("code", function (options, content)
		-- IMPLEMENTATION NOTE:
		-- The \code command came from the url package, though used in plenty of
		-- places. It was referring to the verbatim:font from the verbatim
		-- package, which _should_ be sort of unrelated.
		-- Trying to untangle the things here, by introducing the
		-- definition from the former, but it's of sub-quality...
		-- The ugly -3 size is a HACK of sorts.
		options.family = options.family or "Hack"
		options.size = options.size or SILE.settings:get("font.size") - 3
		SILE.call("font", options, content)
	end)

	self:registerCommand("center", function (_, content)
		if #SILE.typesetter.state.nodes ~= 0 then
			SU.warn([[
				\\center environment started after other nodes in a paragraph

				Content may not be centered as expected.
			]])
		end
		SILE.settings:temporarily(function ()
			local lskip = SILE.settings:get("document.lskip") or SILE.types.node.glue()
			local rskip = SILE.settings:get("document.rskip") or SILE.types.node.glue()
			SILE.settings:set("document.parindent", SILE.types.node.glue())
			SILE.settings:set("current.parindent", SILE.types.node.glue())
			SILE.settings:set("document.lskip", SILE.types.node.hfillglue(lskip.width.length))
			SILE.settings:set("document.rskip", SILE.types.node.hfillglue(rskip.width.length))
			SILE.settings:set("typesetter.parfillskip", SILE.types.node.glue())
			SILE.settings:set("document.spaceskip", SILE.types.length("1spc", 0, 0))
			SILE.process(content)
			SILE.call("par")
		end)
	end, "Typeset its contents in a centered block (keeping margins).")

  	-- Define a command \highlight to read a file and syntax-highlight its content
	-- Mandatory parameter: src=<file name>
	-- Optional parameter: language=<language> (guessed from the file extension if not specified)
	self:registerCommand("highlight", function (options, _)
		-- Load the file
		local src = SU.required(options, "src", "file name")
		local source = SILE.resolveFile(src)
		if not source then
			SU.error("Can't find file " .. src)
		end
		local file = io.open(source, "rb")
		if not file then
			SU.error("Can't open file " .. src)
		end
		local content = file:read "*a"
		file:close()
		local language = options.language or pl.path.extension(src):sub(2)
		io.stderr:write("<" .. src .. "> as " .. language .. "\n")
		SILE.call("bigskip")
		SILE.call("nobreak")
		-- Low-level HACK: pass read content as if raw
		SILE.rawHandlers.highlight({ language = language }, { content })
		SILE.call("allowbreak")
		SILE.call("bigskip")
	end, "Syntax highlight externally included source")

	self:registerCommand("handouts:sectioning", function (options, content)
		local level = SU.required(options, "level", "handouts:sectioning")
	end, "TBD")

	self:registerCommand("handouts:chapter:post", function (_, _)
		SILE.call("par")
		SILE.call("noindent")
	end, "TBD")

	self:registerCommand("handouts:section:post", function (_, _)
		SILE.process({  " " })
	end, "TBD")

	self:registerCommand("handouts:subsection:post", function (_, _)
		SILE.process({  " " })
	end, "TBD")

	self:registerCommand("header", function (options, content)
		SILE.call("eject")
		SILE.call("noindent")
		SILE.call("font", { weight = 800, size = "14pt" }, content)
		SILE.call("bigskip")
	end, "Start a new page with a header")

	self:registerCommand("chapter", function (options, content)
		SILE.call("par")
		SILE.call("noindent")
		SILE.call("handouts:chapterfont", {}, function ()
			SILE.call("handouts:sectioning", {
				level = 1,
				msg = "handouts-chapter-title"
			}, content)
		end)
		local lang = SILE.settings:get("document.language")
		local postcmd = "handouts:chapter:post"
		if SILE.Commands[postcmd .. ":" .. lang] then
			postcmd = postcmd .. ":" .. lang
		end
		SILE.call(postcmd)
		SILE.call("handouts:chapterfont", {}, content)
		SILE.call("novbreak")
		SILE.call("par")
		SILE.call("novbreak")
		SILE.call("bigskip")
		SILE.call("novbreak")
		SILE.call("noindent")
	end, "Begin a new chapter")

	self:registerCommand("section", function (options, content)
		SILE.call("par")
		SILE.call("noindent")
		SILE.call("bigskip")
		SILE.call("goodbreak")
		SILE.call("handouts:sectionfont", {}, function ()
			SILE.call("handouts:sectioning", {
				level = 2,
				msg = "handouts-section-font"
			}, content)
			local lang = SILE.settings:get("document.language")
			local postcmd = "handouts:section:post"
			if SILE.Commands[postcmd .. ":" .. lang] then
					postcmd = postcmd .. ":" .. lang
			end
			SILE.call(postcmd)
			SILE.process(content)
		end)
		SILE.call("par")
		SILE.call("novbreak")
		SILE.call("smallskip")
		SILE.call("novbreak")
		SILE.call("noindent")
	end, "Begin a new section")

	self:registerCommand("subsection", function (options, content)
		SILE.call("par")
		SILE.call("noindent")
		SILE.call("medskip")
		SILE.call("goodbreak")
		SILE.call("handouts:subsectionfont", {}, function ()
			SILE.call("handouts:sectioning", {
				numbering = options.numbering,
				toc = options.toc,
				level = 3,
			}, content)
			local lang = SILE.settings:get("document.language")
			local postcmd = "handouts:subsection:post"
			if SILE.Commands[postcmd .. ":" .. lang] then
				postcmd = postcmd .. ":" .. lang
			end
			SILE.call(postcmd)
			SILE.process(content)
		end)
		SILE.call("par")
		SILE.call("novbreak")
		SILE.call("smallskip")
		SILE.call("novbreak")
		-- English typography (notably) expects the first paragraph under a section
		-- not to be indented. Frenchies, don't use this class :)
		SILE.call("noindent")
   	end, "Begin a new subsection")

	self:registerCommand("handouts:chapterfont", function (_, content)
		SILE.call("font", { weight = 800, size = "22pt" }, content)
	end, "Set font for chapter heading")

	self:registerCommand("handouts:sectionfont", function (_, content)
		SILE.call("font", { weight = 800, size = "15pt" }, content)
	end, "Set font for section heading")

	self:registerCommand("handouts:subsectionfont", function (_, content)
		SILE.call("font", { weight = 800, size = "12pt" }, content)
	end, "Set font for section heading")

	self:registerCommand("place", function (options, content)
		---SILE.process({  " " })
		local framename = "place" .. SILE.scratch.counters.place.value
		local top = SU.required(options, "top", "place")
		local bottom = SU.required(options, "bottom", "place")
		local left = SU.required(options, "left", "place")
		local right = SU.required(options, "right", "place")
		io.stderr:write("Placing frame [" .. framename .. "]\n")
		SILE.call("frame", { id = framename, top = top, bottom = bottom, left = left, right = right }, " ")
		SILE.call("typeset-into", { frame = framename }, content)
		SILE.scratch.counters.place.value = SILE.scratch.counters.place.value + 1
	end, "Put content at a specific place on a page")
end

return class
