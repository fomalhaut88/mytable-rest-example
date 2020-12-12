#!/bin/bash

THREADS=8
CONNECTIONS=100
DURATION=5s

wrk -t$THREADS -c$CONNECTIONS -d$DURATION http://localhost:8000/version
wrk -t$THREADS -c$CONNECTIONS -d$DURATION -s wrk-post-data.lua http://localhost:8000/insert
wrk -t$THREADS -c$CONNECTIONS -d$DURATION http://localhost:8000/get/1
wrk -t$THREADS -c$CONNECTIONS -d$DURATION -s wrk-post-data.lua http://localhost:8000/update
