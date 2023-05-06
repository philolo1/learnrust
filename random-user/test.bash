#!/bin/bash

command_to_run="./target/release/random-user"

for i in {1..100}; do
  $command_to_run &
done

wait
