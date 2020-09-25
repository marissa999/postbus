#!/bin/sh
psql -d "postgres://postgres:postgres@localhost:5432/postbus" -f schema.sql
