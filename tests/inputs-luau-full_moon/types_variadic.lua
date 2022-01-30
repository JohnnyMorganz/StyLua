type Foo = (...number) -> ()
type Baz = (string, ...Foo) -> ...Foo
type Bar = (...number) -> (string, ...number) -> ...any

function bar(...: number): ...number | string
end

local Boo = {}
function Boo:f(name: string, ...: number): () -> (...number) -> ()
  return function()
    return function(_x: string, ...: Foo) end
  end
end