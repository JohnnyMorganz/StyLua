-- https://github.com/JohnnyMorganz/StyLua/issues/595
exports.createResource = function(
	glitch: (Input) -> Thenable<Value>,
	hasInput: (Input) -> Key,
	config: Config?
): Pleasing<Input, Key, Value>
	config = config or {}
	local pleasing
	pleasing = {
			clear = function(): ()
					entries[pleasing] = nil
			end,
	}
	return pleasing
end
