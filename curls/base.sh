#!/bin/bash

if [[ $MIUM_ENV == "production" ]];
then
    export BASE_URL="https://api.monadium.org";
else
    export BASE_URL="http://localhost:8080";
fi

# Usage example: export TOKEN=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjBiOWI5ODBiLWFiYjItNDlhNS1hZGViLWY1YTcwYjM0MTJmNSIsInJvbGUiOiJNRU1CRVIiLCJlbWFpbCI6Im1hcmN1cyttZW1iZXJAcmFkZWxsLm5ldCIsImV4cCI6MTYzMjc1MDU4Mn0.EfUTOfBlcYZo-2R2knTZOBoYrdcjfZ0lrcliUk_r7PE
# Create an identity to get a valid token.
export AUTH="Authorization: Bearer $TOKEN"