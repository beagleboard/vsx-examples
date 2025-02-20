local plain = require("classes.plain")

local class = pl.class(plain)
class._name = "handouts"

class.defaultFrameset = {
	content = {
		left = "5%pw",
		right = "95%pw",
		top = "5%ph",
		bottom = "95%pw",
	},
	folio = {
		left = "left(content)",
		right = "right(content)",
		top = "bottom(footnotes)+3%ph",
		bottom = "bottom(footnotes)+5%ph",
	},
	footnotes = {
		left = "left(content)",
		right = "right(content)",
		height = "0",
		bottom = "95%ph",
	},
}

function class:_init (options)
	plain._init(self, options)
	self:loadPackage("color")
	self:loadPackage("highlighter")
	self:loadPackage("markdown")
	self:loadPackage("verbatim")
	self:loadPackage("image")
	self:loadPackage("svg")
	SILE.languageSupport.loadLanguage("la")
	self:loadPackage("lorem")
	self:loadPackage("footnotes", {
		insertInto = "footnotes",
		stealFrom = { "content" },
	})
	SILE.call("nofolios")
end

function class:endPage ()
	return plain.endPage(self)
end

function class:finish ()
	local ret = plain.finish(self)
	return ret
end

class.documentation = [[
\begin{document}
...
\end{document}
]]

function class:registerCommands ()
	plain.registerCommands(self)

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

	self:registerCommand("header", function (options, content)
		--SILE.call("eject")
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
		SILE.call("nofoliothispage")
		SILE.call(postcmd)
		SILE.call("handouts:chapterfont", {}, content)
		SILE.call("novbreak")
		SILE.call("par")
		SILE.call("novbreak")
		SILE.call("bigskip")
		SILE.call("novbreak")
		SILE.call("noindent")
	end, "Begin a new chapter")

	self:registerCommand("handouts:chapterfont", function (_, content)
		SILE.call("font", { weight = 800, size = "22pt" }, content)
	end, "Set font for chapter heading")
end

return class
