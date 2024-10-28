package main

import (
	"context"
	"fmt"

	"dagger/mittlife-cycles/internal/dagger"
)

const LocalDevServerVersion = "latest"

/* TODO: make integration tests dagger native
func (m *MittlifeCycles) TestIntegrationFuture(
	ctx context.Context,
	source *dagger.Directory,
) (string, error) {
	example := "simple"
	dotEnv := getEnvFile(source, example)

	executable := m.BuildExample(ctx, source, example)

	// Need two of those because dependencies between services have to be a DAG (directed acyclic graph)
	// problem: key serial is random
	// waiting for: https://github.com/dagger/dagger/issues/8590
	localDevServiceKeyProvider := localDevContainer(dotEnv).AsService()

	exampleService := baseServerContainer(executable, dotEnv).
		WithExposedPort(8090).
		WithServiceBinding("key-provider", localDevServiceKeyProvider).
		WithExec([]string{"/server"}).
		AsService()

	localDevService := localDevContainer(dotEnv).
		WithServiceBinding("example-service", exampleService).
		AsService()

	return buildIntegrationTestRunner(
		source.Directory("integration"),
		localDevService,
	).Stdout(ctx)
}
*/

func (m *MittlifeCycles) SimpleExampleService(
	ctx context.Context,
	source *dagger.Directory,
	localDevService *dagger.Service,
) *dagger.Service {
	example := "simple"
	dotEnv := getEnvFile(source, example)

	executable := m.BuildExample(ctx, source, example)

	return baseServerContainer(executable, dotEnv).
		WithServiceBinding("local-dev", localDevService).
		WithExposedPort(8090).
		WithExec([]string{"/server"}).
		AsService()
}

func (m *MittlifeCycles) LocalDevService(
	source *dagger.Directory,
	exampleService *dagger.Service,
) *dagger.Service {
	example := "simple"
	dotEnv := getEnvFile(source, example)

	return dag.Container().
		From("mittwald/marketplace-local-dev-server:"+LocalDevServerVersion).
		WithFile(".env", dotEnv).
		WithServiceBinding("example-service", exampleService).
		AsService()
}

func getEnvFile(source *dagger.Directory, example string) *dagger.File {
	return source.File(fmt.Sprintf("examples/%s/.env", example))
}

func localDevContainer(dotEnv *dagger.File) *dagger.Container {
	return dag.Container().
		From("mittwald/marketplace-local-dev-server:"+LocalDevServerVersion).
		WithFile(".env", dotEnv)
}

func (m *MittlifeCycles) DriveIntegrationTests(
	ctx context.Context,
	source *dagger.Directory,
	localDevService *dagger.Service,
) (string, error) {
	return buildIntegrationTestRunner(source, localDevService).
		Stdout(ctx)
}

func buildIntegrationTestRunner(
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
