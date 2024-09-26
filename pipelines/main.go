package main

import (
	"context"

	"dagger/mittlife-cycles/internal/dagger"
)

const (
	RustVersion = "1.81"
	GoVersion   = "1.23"
)

type MittlifeCycles struct{}

// BuildAndTestAll runs a complete pipeline, that verifies that the library and selected examples work correctly
func (m *MittlifeCycles) BuildAndTestAll(
	ctx context.Context,
	source *dagger.Directory,
) error {
	_, err := m.Check(ctx, source)
	if err != nil {
		return err
	}

	_, err = m.Lint(ctx, source)
	if err != nil {
		return err
	}

	_, err = m.Test(ctx, source)
	if err != nil {
		return err
	}

	_, err = m.ExampleSimpleCheck(ctx, source)
	if err != nil {
		return err
	}

	// TODO: integration test

	return nil
}

// Check verifies that the library code compiles
func (m *MittlifeCycles) Check(
	ctx context.Context,
	source *dagger.Directory,
) (string, error) {
	return NewCachedRustBuilder(source).Check(ctx)
}

// Lint verifies that the library code complies with clippy
func (m *MittlifeCycles) Lint(
	ctx context.Context,
	source *dagger.Directory,
) (string, error) {
	return NewCachedRustBuilder(source).Lint(ctx)
}

// Test verifies that the library code tests run successfully
func (m *MittlifeCycles) Test(
	ctx context.Context,
	source *dagger.Directory,
) (string, error) {
	return NewCachedRustBuilder(source).Test(ctx)
}

// ExampleSimpleCheck verifies that the code of the simple example compiles
// given the directory at the root of the project
func (m *MittlifeCycles) ExampleSimpleCheck(
	ctx context.Context,
	source *dagger.Directory,
) (string, error) {
	return NewCachedRustBuilder(
		source,
		WithWorkdir("examples/simple"),
	).Check(ctx)
}
