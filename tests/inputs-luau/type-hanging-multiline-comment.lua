export type CoverageReporterWithOptions<K> =
	Array<string | Object> --[[ [K, Partial<ReportOptions[K]>] ]]
	| nil
