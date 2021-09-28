#!/bin/bash

EMAIL=$1
PASSWORD=$2

# You need to source this by calling it with: . ../curls/identities/set_token.sh
export TOKEN=$(./curls/identities/create.sh $EMAIL $PASSWORD | jq '.jwt' | tr -d '"')
