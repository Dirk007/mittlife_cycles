package main

import (
	"context"
	"dagger/mittlife-cycles/internal/dagger"
)

// BuildExample builds the executable of an example
func (m *MittlifeCycles) BuildExample(
	ctx context.Context,
	source *dagger.Directory,
	example string,
) *dagger.File {
	return CachedRustBuilder{source}.BuildExample(example)
}

// CheckExamples verifies that all examples compile
func (m *MittlifeCycles) CheckExamples(
	ctx context.Context,
	source *dagger.Directory,
) error {
	return ForAllExamples(ctx, source,
		func(builder CachedRustBuilder, exampleName string) error {
			_, err := builder.CheckExample(ctx, exampleName)
			return err
		},
	)
}

// LintExamples verifies that the code of all examples complies with clippy
func (m *MittlifeCycles) LintExamples(
	ctx context.Context,
	source *dagger.Directory,
) error {
	return ForAllExamples(ctx, source,
		func(builder CachedRustBuilder, exampleName string) error {
			_, err := builder.LintExample(ctx, exampleName)
			return err
		},
	)
}

// TestExamples verifies that the unit tests of every example run successfully
func (m *MittlifeCycles) TestExamples(
	ctx context.Context,
	source *dagger.Directory,
) error {
	return ForAllExamples(ctx, source,
		func(builder CachedRustBuilder, exampleName string) error {
			_, err := builder.LintExample(ctx, exampleName)
			return err
		},
	)
}

func ForAllExamples(
	ctx context.Context,
	source *dagger.Directory,
	action func(CachedRustBuilder, string) error,
) error {
	exampleNames, err := source.Entries(ctx, dagger.DirectoryEntriesOpts{
		Path: "examples",
	})
	if err != nil {
		return err
	}

	builder := CachedRustBuilder{source}
	for _, exampleName := range exampleNames {
		if err := action(builder, exampleName); err != nil {
			return err
		}
	}

	return nil
}
