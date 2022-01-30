function foo<x, y>()
end

local function bar<x>()
end

export type Foo = {
	bar: <T>(
		a: T,
		b: nil | number | boolean
	) -> T,
}

export type Foo = {
	bar: <T>(
		a: T,
		b: nil | number | boolean
	) -> ((arg0: T) -> ())?,
}

baz = function<T>(a: T, b: number | boolean | nil): T
end