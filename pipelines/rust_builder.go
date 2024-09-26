package main

import (
	"context"

	"dagger/mittlife-cycles/internal/dagger"
)

type CachedRustBuilder struct {
	source  *dagger.Directory
	workdir *string
}

type CachedRustBuilderOption func(*CachedRustBuilder)

func WithWorkdir(workdir string) CachedRustBuilderOption {
	return func(crb *CachedRustBuilder) {
		crb.workdir = &workdir
	}
}

func NewCachedRustBuilder(
	source *dagger.Directory,
	options ...CachedRustBuilderOption,
) CachedRustBuilder {
	builder := CachedRustBuilder{
		source: source,
	}

	for _, o := range options {
		o(&builder)
	}

	return builder
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

	workdir := "/src"
	if c.workdir != nil {
		workdir = workdir + "/" + *c.workdir
	}

	return dag.Container().
		From("rust:"+RustVersion).
		WithExec([]string{"rustup", "component", "add", "clippy"}).

		// Source Code
		WithDirectory("/src", source).
		WithWorkdir(workdir).

		// Caches
		WithMountedCache("/cache/cargo", dag.CacheVolume("rust-packages")).
		WithEnvVariable("CARGO_HOME", "/cache/cargo").
		WithMountedCache("target", dag.CacheVolume("rust-target"))
}
