package main

import (
	"dagger/mittlife-cycles/internal/dagger"
)

const AlpineVersion = "3.20"

func baseServerContainer(
	executable *dagger.File,
	dotEnv *dagger.File,
) *dagger.Container {
	return dag.Container().
		From("alpine:"+AlpineVersion).

		// Application
		WithFile("/.env", dotEnv).
		WithFile("/server", executable)
}
