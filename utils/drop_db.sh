#!/bin/bash

# Load database name and utility functions
source config.shlib
source data.shlib

psql $DSN -c "DROP DATABASE ${PG_DATABASE}"
