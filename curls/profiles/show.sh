#!/bin/bash

. ./curls/base.sh

ID=$1

curl \
-H "Content-Type: application/json" \
-H "$AUTH" \
$BASE_URL/profiles/show/$ID
