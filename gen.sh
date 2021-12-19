docker run --rm -v "${PWD}:/local" --user $(id -u):$(id -g)  openapitools/openapi-generator-cli:v5.3.0 generate \
    --skip-validate-spec \
    -i https://finnhub.io/static/swagger.json \
    -g rust \
    -o /local/finnhub-openapi-gen-rs \
    --additional-properties=packageName=finnhub-openapi-gen-rs
