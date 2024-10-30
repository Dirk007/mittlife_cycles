package main

import (
	"context"
	"fmt"

	"dagger/mittlife-cycles/internal/dagger"
)

const LocalDevServerVersion = "latest"

// TestIntegration runs integration tests on the library
func (m *MittlifeCycles) TestIntegration(
	ctx context.Context,
	source *dagger.Directory,
) (string, error) {
	example := "simple"
	dotEnv := getEnvFile(source, example)

	executable := m.BuildExample(ctx, source, example)

	_, err := baseServerContainer(executable, dotEnv).
		WithExposedPort(8090).
		WithExec([]string{"/server"}).
		AsService().
		WithHostname("example-service").
		Start(ctx)
	if err != nil {
		return "", nil
	}

	_, err = localDevContainer(dotEnv).
		AsService().
		WithHostname("local-dev").
		Start(ctx)
	if err != nil {
		return "", nil
	}

	return integrationTestRunner(source.Directory("integration")).Stdout(ctx)
}

func getEnvFile(source *dagger.Directory, example string) *dagger.File {
	return source.File(fmt.Sprintf("examples/%s/.env", example))
}

func localDevContainer(dotEnv *dagger.File) *dagger.Container {
	return dag.Container().
		From("mittwald/marketplace-local-dev-server:"+LocalDevServerVersion).
		WithFile(".env", dotEnv)
}

func integrationTestRunner(source *dagger.Directory) *dagger.Container {
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
		WithExec([]string{"go", "test", "-count=1", "./..."})
}
