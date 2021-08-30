#!/bin/bash

. ./curls/base.sh

curl \
-H "Content-Type: application/json" \
-H "$AUTH" \
$BASE_URL/identities/list
