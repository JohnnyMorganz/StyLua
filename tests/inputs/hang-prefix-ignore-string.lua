-- https://github.com/JohnnyMorganz/StyLua/issues/508
exports.ScalarLeafsRule = function(context)
	return {
			Field = function(_self, node)
					if type_ then
							if not selectionSet then
									context:reportError(
											GraphQLError.new(
													('Field "%s" of type "%s" must have a selection of subfields. Did you mean "%s { ... }"?'):format(
															fieldName,
															typeStr,
															fieldName
													),
													node
											)
									)
							end
					end
			end,
	}
end
