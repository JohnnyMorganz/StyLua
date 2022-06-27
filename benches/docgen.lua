-- https://github.com/neovim/nvim-lspconfig/blob/master/scripts/docgen.lua
-- https://github.com/neovim/nvim-lspconfig/blob/master/LICENSE.md
local preamble_parts = make_parts {
	function()
	  if docs.description and #docs.description > 0 then
		return docs.description
	  end
	end,
	function()
	  local package_json_name = util.path.join(tempdir, template_name .. '.package.json')
	  if docs.package_json then
		if not util.path.is_file(package_json_name) then
		  os.execute(string.format('curl -v -L -o %q %q', package_json_name, docs.package_json))
		end
		if not util.path.is_file(package_json_name) then
		  print(string.format('Failed to download package.json for %q at %q', template_name, docs.package_json))
		  os.exit(1)
		  return
		end
		local data = fn.json_decode(readfile(package_json_name))
		-- The entire autogenerated section.
		return make_section(0, '\n', {
		  -- The default settings section
		  function()
			local default_settings = (data.contributes or {}).configuration
			if not default_settings.properties then
			  return
			end
			-- The outer section.
			return make_section(0, '\n', {
			  'This server accepts configuration via the `settings` key.',
			  '<details><summary>Available settings:</summary>',
			  '',
			  -- The list of properties.
			  make_section(
				0,
				'\n\n',
				sorted_map_table(default_settings.properties, function(k, v)
				  local function tick(s)
					return string.format('`%s`', s)
				  end
				  local function bold(s)
					return string.format('**%s**', s)
				  end

				  -- https://github.github.com/gfm/#backslash-escapes
				  local function excape_markdown_punctuations(str)
					local pattern =
					  '\\(\\*\\|\\.\\|?\\|!\\|"\\|#\\|\\$\\|%\\|\'\\|(\\|)\\|,\\|-\\|\\/\\|:\\|;\\|<\\|=\\|>\\|@\\|\\[\\|\\\\\\|\\]\\|\\^\\|_\\|`\\|{\\|\\\\|\\|}\\)'
					return fn.substitute(str, pattern, '\\\\\\0', 'g')
				  end

				  -- local function pre(s) return string.format("<pre>%s</pre>", s) end
				  -- local function code(s) return string.format("<code>%s</code>", s) end
				  if not (type(v) == 'table') then
					return
				  end
				  return make_section(0, '\n', {
					'- ' .. make_section(0, ': ', {
					  bold(tick(k)),
					  function()
						if v.enum then
						  return tick('enum ' .. inspect(v.enum))
						end
						if v.type then
						  return tick(table.concat(tbl_flatten { v.type }, '|'))
						end
					  end,
					}),
					'',
					make_section(2, '\n\n', {
					  { v.default and 'Default: ' .. tick(inspect(v.default, { newline = '', indent = '' })) },
					  { v.items and 'Array items: ' .. tick(inspect(v.items, { newline = '', indent = '' })) },
					  { excape_markdown_punctuations(v.description) },
					}),
				  })
				end)
			  ),
			  '',
			  '</details>',
			})
		  end,
		})
	  end
	end,
}
