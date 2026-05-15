-- Issue #350: leading | in generic type arguments
type Test = Box<
    | string
    | number
>

type Test2 = Box<| string | number>

-- Issue #311 comment: leading & in generic type arguments
type Test3 = Generic<& boolean & string>

type Test4 = Map<
    & string & number,
    | boolean | nil
>
