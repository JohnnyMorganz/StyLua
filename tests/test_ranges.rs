use stylua_lib::{format_code, Config, OutputVerification, Range};

fn format_range(input: &str, range: Range) -> String {
    format_code(
        input,
        Config::default(),
        Some(range),
        OutputVerification::None,
    )
    .unwrap()
}

fn format(input: &str) -> String {
    let start_point = input.find("||");
    let end_point = input.rfind("||");

    format_code(
        &input.replace("||", ""),
        Config::default(),
        Some(Range::from_values(start_point, end_point)),
        OutputVerification::None,
    )
    .unwrap()
}

#[test]
fn test_default() {
    insta::assert_snapshot!(
        format(
            r###"||local foo     =      bar||
local bar   =     baz
            "###,
        ),
        @r###"
    local foo = bar
    local bar   =     baz

    "###
    );
}

#[test]
fn test_ignore_last_stmt() {
    insta::assert_snapshot!(
        format(
            r###"||f||unction foo()
    return bar
end"###
        ),
    @r###"
    function foo()
        return bar
    end
    "###);
}

#[test]
fn test_dont_modify_eof() {
    insta::assert_snapshot!(
        format(
            r###"||local foo     =      bar||
local bar   =     baz




            "###
        ),
    @r###"

    local foo = bar
    local bar   =     baz





    "###);
}

#[test]
fn test_incomplete_range() {
    insta::assert_snapshot!(
        format(
            r###"||local||    fooo =    bar"###
        ),
    @r###"

    local    fooo =    bar

    "###);
}

#[test]
#[cfg_attr(
    all(debug_assertions, feature = "luau"),
    ignore = "fails in debug mode"  // TODO: https://github.com/Kampfkarren/full-moon/issues/140
)]
fn test_large_example() {
    insta::assert_snapshot!(
        format(
            r#"||if string.sub(msg, 1, 8) == "setgrav/" then
danumber = nil for i = 9, 100 do if string.sub(msg, i, i) == "/" then danumber = i break end end if danumber == nil then
return end local player = findplayer(string.sub(msg, 9, danumber - 1), speaker)
if player == 0 then return end for i = 1, #player do if player[i].Character ~= nil then
local torso = player[i].Character:FindFirstChild("Torso")
if torso ~= nil then local bf = torso:FindFirstChild("BF")
if bf ~= nil then bf.force = Vector3.new(0, 0, 0)
else local bf = Instance.new("BodyForce")
bf.Name = "BF" bf.force = Vector3.new(0, 0, 0)
bf.Parent = torso end local c2 = player[i].Character:GetChildren()
for i = 1, #c2 do if c2[i].className == "Part" then
torso.BF.force = torso.BF.force
+ Vector3.new(0, c2[i]:getMass() * -string.sub(msg, danumber + 1), 0)
end end end end end end||

if string.sub(msg, 1, 5) == "trip/" then local player = findplayer(string.sub(msg, 6), speaker)
if player ~= 0 then for i = 1, #player do
if player[i].Character ~= nil then
local torso = player[i].Character:FindFirstChild("Torso")
if torso ~= nil then torso.CFrame = CFrame.new(torso.Position.x, torso.Position.y, torso.Position.z, 0, 0, 1, 0, -1, 0, 1, 0, 0) --math.random(),math.random(),math.random(),math.random(),math.random(),math.random(),math.random(),math.random(),math.random()) -- i like the people being upside down better.
end end end end end
            "#
        ),
    @r###"

    if string.sub(msg, 1, 8) == "setgrav/" then
    	danumber = nil
    	for i = 9, 100 do
    		if string.sub(msg, i, i) == "/" then
    			danumber = i
    			break
    		end
    	end
    	if danumber == nil then
    		return
    	end
    	local player = findplayer(string.sub(msg, 9, danumber - 1), speaker)
    	if player == 0 then
    		return
    	end
    	for i = 1, #player do
    		if player[i].Character ~= nil then
    			local torso = player[i].Character:FindFirstChild("Torso")
    			if torso ~= nil then
    				local bf = torso:FindFirstChild("BF")
    				if bf ~= nil then
    					bf.force = Vector3.new(0, 0, 0)
    				else
    					local bf = Instance.new("BodyForce")
    					bf.Name = "BF"
    					bf.force = Vector3.new(0, 0, 0)
    					bf.Parent = torso
    				end
    				local c2 = player[i].Character:GetChildren()
    				for i = 1, #c2 do
    					if c2[i].className == "Part" then
    						torso.BF.force = torso.BF.force
    							+ Vector3.new(0, c2[i]:getMass() * -string.sub(msg, danumber + 1), 0)
    					end
    				end
    			end
    		end
    	end
    end

    if string.sub(msg, 1, 5) == "trip/" then local player = findplayer(string.sub(msg, 6), speaker)
    if player ~= 0 then for i = 1, #player do
    if player[i].Character ~= nil then
    local torso = player[i].Character:FindFirstChild("Torso")
    if torso ~= nil then torso.CFrame = CFrame.new(torso.Position.x, torso.Position.y, torso.Position.z, 0, 0, 1, 0, -1, 0, 1, 0, 0) --math.random(),math.random(),math.random(),math.random(),math.random(),math.random(),math.random(),math.random(),math.random()) -- i like the people being upside down better.
    end end end end end

    "###);
}

#[test]
fn test_nested_range() {
    insta::assert_snapshot!(
        format(
            r#"local my_function  =  function()
    ||local nested_statement    =  "foobar"||
end
"#
        ),
    @r###"

    local my_function  =  function()
    	local nested_statement = "foobar"
    end
    "###);
}

#[test]
fn test_nested_range_local_function() {
    insta::assert_snapshot!(
        format(
            r#"local function test()
    call "hello"
    ||call    { x     =   y}
    local   z   = 1    + 3    - (2 / 3)||
end
"#
        ),
    @r###"
    local function test()
        call "hello"
    	call({ x = y })
    	local z = 1 + 3 - (2 / 3)
    end
    "###);
}

#[test]
fn test_nested_range_while() {
    insta::assert_snapshot!(
        format(
            r###"while     true     do
    ||local    z   = 2||
    end
"###
        ),
    @r###"
    while     true     do
    	local z = 2
        end
    "###);
}

#[test]
fn test_nested_range_repeat() {
    insta::assert_snapshot!(
        format(
            r###"repeat||
    local    z   =     2||
until    true
"###
        ),
    @r###"
    repeat
    	local z = 2
    until    true
    "###);
}

#[test]
fn test_nested_range_do() {
    insta::assert_snapshot!(
        format(
            r###"do
    ||local    z   =     2||
end
"###
        ),
    @r###"
    do
    	local z = 2
    end
    "###);
}

#[test]
fn test_nested_range_generic_for() {
    insta::assert_snapshot!(
        format(
            r###"for    i,    v  in pairs(x) do
    ||local    z   =     2||
end
"###
        ),
    @r###"
    for    i,    v  in pairs(x) do
    	local z = 2
    end
    "###);
}

#[test]
fn test_nested_range_else_if() {
    insta::assert_snapshot!(
        format(
            r###"if   x    and  y  then
    local   p  = q
elseif      c - d > 2  then
    ||local    z   =     2||
end
"###
        ),
    @r###"
    if   x    and  y  then
        local   p  = q
    elseif      c - d > 2  then
    	local z = 2
    end
    "###);
}

#[test]
fn test_nested_range_function_call() {
    insta::assert_snapshot!(
        format(
            r###"call   (function    ()
    ||local    z   =   5||
            end)
"###
        ),
    @r###"
    call   (function    ()
    	local z = 5
                end)
    "###);
}

#[test]
fn test_nested_range_function_call_table() {
    insta::assert_snapshot!(
        format(
            r###"call   {   x  =   function()
   ||local    z   =    2||
end}
"###
        ),
    @r###"
    call   {   x  =   function()
    	local z = 2
    end}
    "###);
}

#[test]
fn test_nested_range_table_1() {
    insta::assert_snapshot!(
        format(
            r###"local    z    = {
                      function()
                                ||local z      =  5||
                    end
            }
"###
        ),
    @r###"
    local    z    = {
                          function()
    	local z = 5
                        end
                }
    "###);
}

#[test]
fn test_nested_range_table_2() {
    insta::assert_snapshot!(
        format(
            r###"local    z    = {
                      [(function()
                      ||return     random_func  ()||

            end)()] = true
            }
"###
        ),
    @r###"
    local    z    = {
                          [(function()
    	return random_func()

                end)()] = true
                }
    "###);
}

#[test]
fn test_nested_range_binop() {
    insta::assert_snapshot!(
        format(
            r###"local    z    =(    1     +  (function()
        ||local    p   =  q||
        end)())
"###
        ),
    @r###"
    local    z    =(    1     +  (function()
    	local p = q
            end)())
    "###);
}

#[test]
fn test_no_range_start() {
    insta::assert_snapshot!(
        format_range(
            r###"local     z   =   2
local   e    = 5
"###,
            Range::from_values(None, Some(20))
        ),
    @r###"
    local z = 2
    local   e    = 5
    "###);
}

#[test]
fn test_no_range_end() {
    insta::assert_snapshot!(
        format_range(
            r###"local     z   =   2
local   e    = 5
"###,
            Range::from_values(Some(20), None)
        ),
    @r###"

    local     z   =   2
    local e = 5
    "###);
}
