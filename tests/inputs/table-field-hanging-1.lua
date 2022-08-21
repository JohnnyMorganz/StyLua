-- https://github.com/JohnnyMorganz/StyLua/issues/542
-- https://github.com/JohnnyMorganz/StyLua/issues/541
local thisIsATable = {
	CreateAnElementFromThisTable = SomethingIsSelected and getTheSelectedThing(TheSelectedItem) or getTheSelectedThing(NoItemSelected)
}
