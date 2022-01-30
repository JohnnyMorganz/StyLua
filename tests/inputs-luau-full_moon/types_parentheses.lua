function TypeInfo.new(
	getFieldDefFn: (() -> GField<any, any>?)?
)
end

export type Thunk<T> = (() -> T) | T

export type PromiseLike<T> = {
    andThen: (
                ((T) -> T)? | (PromiseLike<T>)?, -- resolve
                ((any) -> () | PromiseLike<T>)? -- reject
        ) -> PromiseLike<T>
}

function GError.new(
	originalError: (Error & { extensions: any? }) -- new syntax
): GError
end

type IProperties = {
	RemoveOnCollision: (string | (IProperties, BasePart, Vector3, Vector3, Enum.Material, number) -> boolean)?,
}