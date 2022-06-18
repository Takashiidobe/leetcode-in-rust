#!/usr/bin/env bash

for i in $(cat files.txt);do
  touch $i
done
