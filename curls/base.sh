#!/bin/bash

if [[ $MIUM_ENV == "production" ]];
then
    export BASE_URL="https://api.monadium.org";
else
    export BASE_URL="http://localhost:8080";
fi

export AUTH="Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjgwNWQ1ZmZmLWQ5OGMtNDk2OS1iZTUwLTVjYjY0YjM1ZWQzYyIsInJvbGUiOiJPV05FUiIsImVtYWlsIjoibWFyY3VzQHJhZGVsbC5uZXQiLCJleHAiOjE2MzI3MzQ1MTB9.6fb90fnR6m2mxkb-kMs3atFG2L2rXtMBJflqQefuF4U"