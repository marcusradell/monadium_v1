#!/bin/bash

if [[ $MIUM_ENV == "production" ]];
then
    export BASE_URL="https://api.monadium.org";
else
    export BASE_URL="http://localhost:8080";
fi

export AUTH="Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6Im1hcmN1c0ByYWRlbGwubmV0IiwiaWQiOiI3MTA3NTJkZi0yYTcxLTRlYWUtOWU4ZS0wM2EwM2Q3M2JlYjQiLCJleHAiOjE2MzA0MjE2ODB9.nCdnex_6k_NAzFLAcPiaQpGB_OftnXIEPLD4Zs5189"