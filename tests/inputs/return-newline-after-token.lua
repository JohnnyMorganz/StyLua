-- https://github.com/JohnnyMorganz/StyLua/issues/605
function foo()
	return
		delta.tag == Band or
		delta.tag == Drum or
		delta.tag == Bass
end

function foo()
	return
		delta.tag == Band or
		delta.tag == Drum or
		delta.tag == Bass or
		delta.tag == Lol or
		delta.tag == Hello or
		delta.tag == Drum or
		delta.tag == Bass
end
