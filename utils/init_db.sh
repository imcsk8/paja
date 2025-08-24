#!/bin/bash

# Load database name and utility functions
source config.shlib
source data.shlib

initialize_db

#psql $DSN -c '\ir ../sql/schema.sql'

#echo "Adding data"
#load_requests
