-- https://github.com/JohnnyMorganz/StyLua/issues/378
export type KindEnum =
	"Name" |
	-- Document
	"Document"
	| "OperationDefinition"
	| "VariableDefinition"
	| "SelectionSet"
	| "Field"
	| "Argument" |
	-- Fragments
	"FragmentSpread"
	| "InlineFragment"
	| "FragmentDefinition"
