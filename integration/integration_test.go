package main

import (
	"testing"

	"github.com/stretchr/testify/suite"
)

type WebhooksTestSuite struct {
	suite.Suite
}

func TestWebhooks(t *testing.T) {
	suite.Run(t, new(WebhooksTestSuite))
}

func (s *WebhooksTestSuite) TestAddingExtensionToContext() {
	s.assertWebhookCall("extensionAddedToContext")
}
