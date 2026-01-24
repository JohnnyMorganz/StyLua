-- https://github.com/JohnnyMorganz/StyLua/issues/1038

type Object1<T = (nil)> = {
	method: (T) -> (),
}

type Object2<T... = (nil)> = {
	method: (T...) -> (),
}

