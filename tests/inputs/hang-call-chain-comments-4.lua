-- https://github.com/JohnnyMorganz/StyLua/issues/873

x = a.b -- comment
{
  y
}

console.dialog = iup.dialog
{
  iup.hbox -- use it to inherit margins
  {
    console.prompt,
  },
  title = "Command:",
}
