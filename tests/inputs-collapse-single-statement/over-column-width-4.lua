-- https://github.com/JohnnyMorganz/StyLua/issues/704
vim.api.nvim_create_user_command('F', function(options) require('greeeeeeeeeeeeeeeeeeeeep').by_fixed(options.args) end, {
	nargs = '+',
	complete = 'file',
  })
