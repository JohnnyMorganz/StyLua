local x = [[
box[0,0;1,1;]
dropdown[4,0.75;6.4;contributor_grouping;]] .. S("Group by name") .. "," .. S("Group by contribution") .. ";" .. mail.selected_idxs.contributor_grouping[name] .. [[;true]
]]
local y = "dropdown[4,0.75;6.4;contributor_grouping;" .. S("Group by name") .. "," .. S("Group by contribution") .. ";" .. mail.selected_idxs.contributor_grouping[name] .. ";true]"
local z = [[
short]] .. foo .. [[bar]]
x = [[
box[0,0;1,1;]
dropdown[4,0.75;6.4;contributor_grouping;]] .. S("Group by name") .. "," .. S("Group by contribution") .. ";" .. mail.selected_idxs.contributor_grouping[name] .. [[;true]
]]
