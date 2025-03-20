#!/bin/bash

python3 -m venv .venv
source .venv/bin/activate

npm -i

make all
make run

