package tree_sitter_noted_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-noted"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_noted.Language())
	if language == nil {
		t.Errorf("Error loading Noted grammar")
	}
}
