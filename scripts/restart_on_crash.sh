#!/bin/bash
# envoke this file from cron

if ! pgrep mordoprojekt-bot >/dev/null 2>&1
  then
    bash ~/dist/deploy.sh
fi