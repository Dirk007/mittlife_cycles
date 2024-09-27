package main

import (
	"dagger/mittlife-cycles/internal/dagger"
)

func baseServerContainer(
	executable *dagger.File,
	dotEnv *dagger.File,
) *dagger.Container {
	return dag.Container().
		From("debian:bookworm-slim").

		// Install Dependencies
		WithExec([]string{"apt-get", "update"}).
		WithExec([]string{"apt-get", "install", "-y", "libssl3", "ca-certificates"}).
		WithExec([]string{"rm", "-rf", "/var/lib/apt/lists/*"}).

		// Application
		WithFile("/.env", dotEnv).
		WithFile("/server", executable)
}
