. ./curls/base.sh

ID=10

curl \
-H "Content-Type: application/json" \
-H "$AUTH" \
$BASE_URL/profiles/show/$ID
