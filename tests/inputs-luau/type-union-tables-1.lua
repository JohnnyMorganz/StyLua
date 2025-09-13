-- https://github.com/JohnnyMorganz/StyLua/issues/958

export type AstExprTableItem =
	| { kind: "list", value: AstExpr, separator: Token<"," | ";">? }
	| { kind: "record", key: string, equals: Token<"=">, value: AstExpr, separator: Token<"," | ";">? }
	| { kind: "general", key: string, equals: Token<"=">, value: AstExpr, separator: Token<"," | ";">? }

export type AstExprTableItem = | { kind: "list", value: AstExpr, separator: Token<"," | ";">? } | {
	kind: "record",
	key: string,
	equals: Token<"=">,
	value: AstExpr,
	separator: Token<"," | ";">?,
} | {
	kind: "general",
	key: string,
	equals: Token<"=">,
	value: AstExpr,
	separator: Token<"," | ";">?,
}
