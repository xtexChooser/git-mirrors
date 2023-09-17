#!/usr/bin/env bash

tiang::target localhost ssh://localhost
tiang::target localhost2 ssh://localhost "$(jo extraData=true)"
