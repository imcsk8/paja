#!/bin/bash

source config.shlib
source data.shlib

echo "Creating 'paja' user"
psql $DSN -c '\ir ../sql/create_user.sql'

dropdb -U postgres -h $PG_HOST -p $PORT $PG_DATABASE
createdb -U postgres -h $PG_HOST -p $PORT -O $PG_USER $PG_DATABASE

initialize_db
