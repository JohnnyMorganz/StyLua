-- Issue #311 comment: leading | and & in function return types
type UnionReturn = () -> | string | number

type IntersectionReturn = () -> & string & number

type NestedReturn = (string) -> | boolean | nil

-- Also valid in function type annotations
local f: () -> | string | number = nil
local g: (number) -> & string & number = nil
