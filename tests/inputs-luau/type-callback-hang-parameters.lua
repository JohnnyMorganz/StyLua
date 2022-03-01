export type ObservableQueryFields<TData, TVariables> = ObservableQueryPick<TData, TVariables> & {
    fetchMore: ((
        _self: any,
        fetchMoreOptions: FetchMoreQueryOptions<TVariables, TData> & FetchMoreOptions<TData, TVariables>
    ) -> Promise<ApolloQueryResult<TData>>) & ((<TData2, TVariables2>(
        _self: any,
        fetchMoreOptions: { query: (DocumentNode | TypedDocumentNode<TData, TVariables>)? } & FetchMoreQueryOptions<TVariables2, TData> & FetchMoreOptions<TData2, TVariables2>
    ) -> Promise<ApolloQueryResult<TData2>>)),
}

type Foo = (
	a: X & -- test
	Y
) -> string

type Foo = () -> X & -- test
Y
