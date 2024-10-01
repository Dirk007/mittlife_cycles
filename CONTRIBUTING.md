## Run Integration Tests

Until dagger supports two-way communication between services, use these manual steps to run integration tests.

1. Start the example service: `dagger call simple-example-service --source=. --local-dev-service=tcp://localhost:8080 up --ports 8090:8090`
2. Start the local dev server: `dagger call local-dev-service --source=. --example-service=tcp://localhost:8090 up --ports 8080:8080`
3. Drive the integration tests: `dagger call drive-integration-tests --source=integration --local-dev-service=tcp://localhost:8080`

