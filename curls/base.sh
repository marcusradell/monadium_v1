#!/bin/bash

if [[ $MIUM_ENV == "production" ]];
then
    export BASE_URL="https://api.monadium.org";
else
    export BASE_URL="http://localhost:8080";
fi

export AUTH="Authorization: Bearer TODO"
