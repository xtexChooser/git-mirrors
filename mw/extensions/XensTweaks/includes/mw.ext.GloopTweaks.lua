local GloopTweaks = {}
local php

function GloopTweaks.filepath( name, width )
	return php.filepath( name, width )
end

function GloopTweaks.setupInterface( options )
	-- Boilerplate
	GloopTweaks.setupInterface = nil
	php = mw_interface
	mw_interface = nil

	-- Register this library in the "mw" global
	mw = mw or {}
	mw.ext = mw.ext or {}
	mw.ext.GloopTweaks = GloopTweaks

	package.loaded['mw.ext.GloopTweaks'] = GloopTweaks
end

return GloopTweaks