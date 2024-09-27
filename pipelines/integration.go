package main

import (
	"context"

	"dagger/mittlife-cycles/internal/dagger"
)

func (m *MittlifeCycles) TestIntegration(
	ctx context.Context,
	source *dagger.Directory,
) (string, error) {
	example := "simple"

	executable := m.BuildExample(ctx, source, example)
	dotEnv := source.File("examples/" + example + "/.env")

	// Need two of those because dependencies between services have to be a DAG (directed acyclic graph)
	// problem: key serial is random
	localDevServiceKeyProvider := dag.Container().
		From("mittwald/marketplace-local-dev-server:1.3.6").
		WithFile(".env", dotEnv).
		AsService()

	exampleService := buildContainerWithEnv(ctx, executable, dotEnv).
		WithServiceBinding("key-provider", localDevServiceKeyProvider).
		AsService()

	localDevService := dag.Container().
		From("mittwald/marketplace-local-dev-server:1.3.6").
		WithServiceBinding("example-service", exampleService).
		WithFile(".env", dotEnv).
		AsService()

	return integrationTestRunner(
		source.Directory("integration"),
		localDevService,
	).Stdout(ctx)
}

func buildContainerWithEnv(
	ctx context.Context,
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
		WithExposedPort(8090).
		WithFile("/app/.env", dotEnv).
		WithFile("/app/server", executable).
		WithWorkdir("/app").
		WithExec([]string{"/app/server"})
}

func integrationTestRunner(
	source *dagger.Directory,
	localDevService *dagger.Service,
) *dagger.Container {
	return dag.Container().
		From("golang:"+GoVersion).

		// Caches
		WithMountedCache("/go/pkg/mod", dag.CacheVolume("go-mod")).
		WithEnvVariable("GOMODCACHE", "/go/pkg/mod").
		WithMountedCache("/go/build-cache", dag.CacheVolume("go-build")).
		WithEnvVariable("GOCACHE", "/go/build-cache").

		// Execute tests
		WithDirectory("/src", source).
		WithWorkdir("/src").
		WithServiceBinding("local-dev", localDevService).
		WithExec([]string{"go", "test", "-count=1", "./..."})
}
