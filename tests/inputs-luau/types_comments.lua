export type IntrospectionNamedTypeRef<
  T, -- TODO: add generic constraints and default types: IntrospectionType = IntrospectionType,
  P
> = {
  kind: any, -- deviation: add this type spec later: $PropertyType<T, 'kind'>,
  name: string,
  ofType: T -- TODO: this field is missing
}