#!/bin/bash
cd system
# cargo run --release -- -i "takes/model_test.take" -t -1


cargo run --bin di --release -- -i "takes/model_test.take" -t -1
