local base = require("packages.base")

local package = pl.class(base)
package._name = "handouts"

function package:_init (options)
	base._init(self)
	self:loadPackage("color")
	self:loadPackage("highlighter")
	self:loadPackage("markdown")
end

package.documentation = [[
\begin{document}
...
\end{document}
]]

function package:registerCommands ()
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
    -- Low-level HACK: pass read content as if raw
    SILE.rawHandlers.highlight({ language = language }, { content })
  end, "Syntax highlight externally included source")
end

return package
