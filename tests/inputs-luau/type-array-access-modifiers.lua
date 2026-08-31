type SingleRead = { read Service }
type SingleWrite = { write Service }

type MultiRead = {
	-- keep this comment
	read Service
}

type MultiWrite = {
	-- and keep this one
	write Service
}

type MultiNested = {
	read {
		Name: string,
		Other: number,
	}
}

type CommentBetweenLine = {
	read -- between
	Service
}

type CommentBetweenBlock = { read --[[between]] Service }

type ExcessWhitespace = { read     Service }

type PropertyAccess = { read Name: string }
type IndexerAccess = { read [number]: Service }
type NoAccess = { Service }

type BoundaryFits = {
	field: { read AccessModifierShapeAccountingBoundaryServiceTypeNameFillingTheAvailableColumnBudgetExactlyHereOkX | B },
}

type BoundaryHangs = {
	field: { read AccessModifierShapeAccountingBoundaryServiceTypeNameFillingTheAvailableColumnBudgetExactlyHereOkXX | B },
}
