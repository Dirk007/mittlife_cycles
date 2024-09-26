package main

import (
	"context"

	"dagger/mittlife-cycles/internal/dagger"
)

type CachedRustBuilder struct {
	source *dagger.Directory
}

func (c CachedRustBuilder) Check(ctx context.Context) (string, error) {
	return c.Container().
		WithExec([]string{"cargo", "check"}).
		Stdout(ctx)
}

func (c CachedRustBuilder) Test(ctx context.Context) (string, error) {
	return c.Container().
		WithExec([]string{"cargo", "test"}).
		Stdout(ctx)
}

func (c CachedRustBuilder) Lint(ctx context.Context) (string, error) {
	return c.Container().
		WithExec([]string{"cargo", "clippy", "--", "-D", "warnings"}).
		Stdout(ctx)
}

func (c CachedRustBuilder) Container() *dagger.Container {
	source := c.source.
		WithoutDirectory("target").
		WithoutDirectory("examples/*/target")

	return dag.Container().
		From("rust:"+RustVersion).
		WithExec([]string{"rustup", "component", "add", "clippy"}).

		// Source Code
		WithDirectory("/src", source).
		WithWorkdir("/src").

		// Caches
		WithMountedCache("/cache/cargo", dag.CacheVolume("rust-packages-library")).
		WithEnvVariable("CARGO_HOME", "/cache/cargo").
		WithMountedCache("target", dag.CacheVolume("rust-target"))
}
