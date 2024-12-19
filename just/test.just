TEST_LOGFILE:="./target/integration_tests.log"

# Run all unit tests and integration tests
[group('test')]
test_all: test integration_test


# Run all unit tests
[group('test')]
test test_name="":
  cargo test {{test_name}}


# Run pocket-ic and state machine integration tests
[group('test')]
integration_test test_name="":
  cargo test {{test_name}} -p integration-tests --no-default-features
