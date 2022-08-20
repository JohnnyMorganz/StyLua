-- https://github.com/JohnnyMorganz/StyLua/issues/524
if ( object == "linebreak" or	--Force a new line
	type(object) == "table" and	--Make sure this is an actual object before checking further.
	((container.flowMaxPerLine and currentPrimaryLine > container.flowMaxPerLine) or	--We went past the max number of columns
		currentSecondaryOffset + object["Get"..primaryDirection](object) > container["Get"..primaryDirection](container)) ) then	--We went past the max pixel width.
end

if ( name and
	((not strictFiltering) and
		( tokenTable[subgroup] or tokenTable[className] or (role and tokenTable[role]) or tokenTable[assignedRole] ) -- non-strict filtering
	) or
		( tokenTable[subgroup] and tokenTable[className] and ((role and tokenTable[role]) or tokenTable[assignedRole]) ) -- strict filtering
) then

end
