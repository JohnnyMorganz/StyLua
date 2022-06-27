-- https://github.com/JohnnyMorganz/StyLua/pull/476#issuecomment-1166663080
local function interpolateVariables(title, template, index)
    return Array.reduce(
        Array.reduce(Object.keys(template), getMatchingKeyPaths(title), {}), -- aka flatMap
        replaceKeyPathWithValue(template),
        title
    ):gsub(
        "%$#", -- ROBLOX deviation: escaped string
        tostring(index),
        1
    )
end
