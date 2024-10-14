package main

import (
	"context"
	"fmt"

	"dagger/mittlife-cycles/internal/dagger"
)

type CachedRustBuilder struct {
	source *dagger.Directory
}

func (c CachedRustBuilder) Check(ctx context.Context) (string, error) {
	return c.Container().
		WithExec([]string{"cargo", "check", "--all-features"}).
		Stdout(ctx)
}

func (c CachedRustBuilder) Test(ctx context.Context) (string, error) {
	return c.Container().
		WithExec([]string{"cargo", "test", "--all-features"}).
		Stdout(ctx)
}

func (c CachedRustBuilder) Lint(ctx context.Context) (string, error) {
	return c.Container().
		WithExec([]string{"cargo", "clippy", "--all-features", "--", "-D", "warnings"}).
		Stdout(ctx)
}

func (c CachedRustBuilder) Build(binaryName string) *dagger.File {
	return c.Container().
		WithExec([]string{"cargo", "build", "--release"}).
		// Without copying, dagger tries to get the binary from the cache
		WithExec([]string{"cp", "target/release/" + binaryName, binaryName}).
		File(binaryName)
}

func (c CachedRustBuilder) CheckExample(
	ctx context.Context,
	example string,
) (string, error) {
	return c.Container().
		WithExec([]string{"cargo", "check", "--example", example}).
		Stdout(ctx)
}

func (c CachedRustBuilder) LintExample(
	ctx context.Context,
	example string,
) (string, error) {
	return c.Container().
		WithExec([]string{
			"cargo", "clippy",
			"--example", example,
			"--", "-D", "warnings",
		}).
		Stdout(ctx)
}

func (c CachedRustBuilder) BuildExample(example string) *dagger.File {
	return c.Container().
		WithExec([]string{"cargo", "build", "--release", "--example", example}).
		// Without copying, dagger tries to get the binary from the cache
		WithExec([]string{"cp", "target/release/examples/" + example, example}).
		File(example)
}

func (c CachedRustBuilder) Container() *dagger.Container {
	source := c.source.
		WithoutDirectory("target").
		WithoutDirectory("examples/*/target")

	return dag.Container().
		From(fmt.Sprintf("rust:%s-alpine", RustVersion)).
		WithExec([]string{"apk", "update"}).
		WithExec([]string{
			"apk", "add", "--no-cache",
			"pkgconfig", "musl-dev",
			"openssl-dev", "openssl-libs-static",
		}).
		WithExec([]string{"rustup", "component", "add", "clippy"}).

		// Source Code
		WithDirectory("/src", source).
		WithWorkdir("/src").

		// Caches
		WithMountedCache("/cache/cargo", dag.CacheVolume("rust-packages")).
		WithEnvVariable("CARGO_HOME", "/cache/cargo").
		WithMountedCache("target", dag.CacheVolume("rust-target"))
}
