#!/bin/bash

trunk build --release
rm -rf $HOME/distribution/*
cp dist/* $HOME/distribution/
