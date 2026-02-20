hurl_opts := "--variables-file hurl_variables --test"

wait-for-api:
    hurl hurl_health.hurl --retry 1 --retry-interval 1000 {{ hurl_opts }}

# run acceptance tests against the running test stack
test *args: wait-for-api
    hurl hurl_tests/*.hurl {{ hurl_opts }} {{ args }}
