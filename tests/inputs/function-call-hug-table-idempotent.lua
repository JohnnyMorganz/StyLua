-- https://github.com/JohnnyMorganz/StyLua/issues/1193
-- A last table argument written on one line, too long to fit, must be formatted the same way
-- as when it is already expanded, so that formatting is idempotent.
assert.are.same({ true, false, false, false },
  { forms.keeps("heading", "identifier"), forms.keeps("heading", "style"),
    forms.keeps("div", "identifier"), forms.keeps("span", "identifier") })
