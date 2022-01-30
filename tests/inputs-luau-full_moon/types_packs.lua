function foo<T...>(param: () -> T...)
end

type Foo<T...> = () -> T...

function bar<T...>(...: T...)
end

type C<S...> = A<number, S...> -- with a generic type pack
type B = A<number, ...string> -- with a variadic type pack
type D = A<number, ()> -- with an empty type pack

type Function<Args..., Return...> = (Args...) -> Return...

type AnyFunction = Function<...any, ...any>

local g: Function<(number, string, ...string), (string, number)> = f