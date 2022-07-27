-- https://github.com/JohnnyMorganz/StyLua/issues/504
local x = {
	FragmentDefinition = function(ref)
		local name, typeCondition, variableDefinitions, directives, selectionSet =
			ref.name, ref.typeCondition, ref.variableDefinitions, ref.directives, ref.selectionSet
		return
		-- Note: fragment variable definitions are experimental and may be changed
		-- or removed in the future.
			("fragment %s%s "):format(
				tostring(name),
				tostring(wrap("(", join(variableDefinitions, ", "), ")"))
			) .. ("on %s %s"):format(
				tostring(typeCondition),
				tostring(wrap("", join(directives, " "), " "))
			) .. tostring(selectionSet)
        end,
}
