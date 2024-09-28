package main

import (
	"bytes"
	"encoding/json"
	"net/http"

	"mittlife_cycles/integration/pkg/pointer"
)

func (s *WebhooksTestSuite) assertWebhookCall(
	webhookType string,
	options ...WebhookCallOption,
) {
	s.T().Helper()

	config := defaultWebhookCallConfiguration()

	for _, o := range options {
		o(&config)
	}

	var requestBody bytes.Buffer
	requestBodyEncoder := json.NewEncoder(&requestBody)
	err := requestBodyEncoder.Encode(config.WebhookCallRequest)
	s.NoError(err, "encoutered error when encoding the webhook call request")

	response, err := http.Post(
		"http://local-dev:8080/internal/calls/"+webhookType,
		"application/json",
		&requestBody,
	)
	s.NoError(err, "encountered error when requesting webhook call")
	defer response.Body.Close()

	s.Equal(config.StatusCode, response.StatusCode)

	var extensionResponse ExtensionResponse
	responseBodyDecoder := json.NewDecoder(response.Body)
	err = responseBodyDecoder.Decode(&extensionResponse)
	s.NoError(err, "encountered error when decoding the extension response")

	s.Equal(config.ExtensionResponse, extensionResponse)
}

type WebhookCallRequest struct {
	ExtensionID               *string  `json:"extensionId"`
	ContributorID             *string  `json:"contributorId"`
	Context                   *string  `json:"context"`
	ContextAggregateID        *string  `json:"contextAggregateId"`
	ExtensionInstanceID       *string  `json:"extensionInstanceId"`
	ConsentedScopes           []string `json:"consentedScopes"`
	ExtensionInstanceDisabled bool     `json:"extensionInstanceDisabled"`
}

type ExtensionResponse struct {
	StatusCode int    `json:"statusCode"`
	Body       string `json:"body"`
	Successful bool   `json:"successful"`
}

type WebhookCallConfiguration struct {
	WebhookCallRequest
	ExtensionResponse
}

func defaultWebhookCallConfiguration() WebhookCallConfiguration {
	return WebhookCallConfiguration{
		WebhookCallRequest: WebhookCallRequest{
			Context: pointer.Of("customer"),
		},
		ExtensionResponse: ExtensionResponse{
			StatusCode: 200,
			Body:       "",
			Successful: true,
		},
	}
}

type WebhookCallOption func(*WebhookCallConfiguration)

func WithExtensionInstanceID(extensionInstanceID string) WebhookCallOption {
	return func(config *WebhookCallConfiguration) {
		config.ExtensionInstanceID = &extensionInstanceID
	}
}
