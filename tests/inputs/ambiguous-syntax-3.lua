local x = call "";
(foo or bar and baz)(bar)

local x = call {};
(foo or bar and baz)(bar)

local x = identifier;
(foo or bar and baz)(bar)

local x = (identifier);
(foo or bar and baz)(bar)

local x = x.y;
(foo or bar and baz)(bar)

local x = x["y"];
(foo or bar and baz)(bar)
