return cframe
	-- Clamp & transform into world space
	* Vector3.new(
		math.clamp(transform.X, -halfSize.X, halfSize.X),
		math.clamp(transform.Y, -halfSize.Y, halfSize.Y),
		math.clamp(transform.Z, -halfSize.Z, halfSize.Z)
	), cframe.Position
