-- https://github.com/JohnnyMorganz/StyLua/issues/514
local function escape(str)
	return (str:gsub("\\", "\\\\"):gsub("(%c)%f[0-9]", longControlCharEscapesssssssssssssssssssss):gsub("%c", shortControlCharEscapes))
end

do
	function dec(data)
		data = string.gsub(data, '[^' .. chars .. '=]', '')
		return (data:gsub('.', function(x)
			if (x == '=') then return '' end
			local r, f = '', (chars:find(x) - 1)
			for i=6,1,-1 do r=r..(f%2^i-f%2^(i-1)>0 and '1' or '0') end
			return r;
		end):gsub('%d%d%d?%d?%d?%d?%d?%d?', function(x)
			if (#x ~= 8) then return '' end
			local c=0
			for i=1,8 do c=c+(x:sub(i,i)=='1' and 2^(8-i) or 0) end
			return string.char(c)
		end))
	end
end
