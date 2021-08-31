#!/bin/bash

. ./curls/base.sh

EMAIL=$1
PASSWORD=$2

json()
{
cat <<EOF
{
    "email": "$EMAIL",
    "password": "$PASSWORD"
}
EOF
}

curl \
-H "Content-Type: application/json" \
-d "$(json)" \
$BASE_URL/identities/sign_in
