-- https://github.com/JohnnyMorganz/StyLua/issues/508
exports.ScalarLeafsRule = function(context)
	return {
			Field = function(_self, node)
					local type_ = context:getType()
					local selectionSet = node.selectionSet
					if type_ then
							if isLeafType(getNamedType(type_)) then
									if selectionSet then
											local fieldName = node.name.value
											local typeStr = inspect(type_)
											context:reportError(
													GraphQLError.new(
															('Field "%s" must not have a selection since type "%s" has no subfields.'):format(
																	fieldName,
																	typeStr
															),
															selectionSet
													)
											)
									end
							elseif not selectionSet then
									local fieldName = node.name.value
									local typeStr = inspect(type_)
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
