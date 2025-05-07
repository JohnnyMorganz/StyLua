local foo = {
	[ [[test]] :: test ] = true,
	[ ([[foo]]) :: test ] = true,
	[ ( [[bar]] ) :: test ] = true,
}

foo[ [[test]] :: test ] = false
foo[ ([[foo]]) :: test ] = false
foo[ ( [[bar]] ) :: test ] = false
