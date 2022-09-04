-- https://github.com/JohnnyMorganz/StyLua/issues/543
-- no need to expand

call(item, --[[param=]] false)

call(--[[ we don't use it ]]true)

call(
	--[[
		this comment spans
		multiple lines
	]]
	false
)

x(
	true,
	90210
	--[[
		color wheel is time-reversed
	]],
	--[[ frobnikate the widget ]]
	false,
	true
	--[[ spin the tesla coils ]]
)
