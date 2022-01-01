-- https://github.com/JohnnyMorganz/StyLua/issues/318
local a = {
	b = -- equals trailing comment
		foo(),
	c -- key trailing comment
		= bar(),
	-- expression leading comment
	"d",
	-- key leading comment
	e -- key trailing comment
	-- equals leading comment
	= -- equals trailing comment
	baz(),


	["f"] = -- equals trailing comment
		foo(),
	["g"] -- key trailing comment
		= bar(),
	-- key leading comment
	["h"] -- key trailing comment
	-- equals leading comment
	= -- equals trailing comment
	baz(),
}

local b = {
    b =  -- a comment breaks it
    {
        c = "d",
    },
}
