-- leading newlines at start of block should be removed even when local/const function has attributes
do

@native
local function foo()
end
end

do

@native
const function bar()
end
end
