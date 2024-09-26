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

// Check verifies that the library code compiles
func (m *MittlifeCycles) Check(
	ctx context.Context,
	source *dagger.Directory,
) (string, error) {
	return CachedRustBuilder{
		source,
	}.Check(ctx)
}

// CheckExampleSimple verifies that the code of the simple example compiles
// given the directory at the root of the project
//func (m *MittlifeCycles) CheckExampleSimple(
//	ctx context.Context,
//	source *dagger.Directory,
//) (string, error) {
//	return CachedRustBuilder{
//		source.Directory("examples/simple"),
//	}.Check(ctx)
//}
