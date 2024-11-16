-- https://github.com/JohnnyMorganz/StyLua/issues/893
type Foo = {
	Status: "loading" -- loading 
	| "error" -- error
	| "success" -- success
}