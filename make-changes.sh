#!/bin/bash

read -p "Enter commit message: " commit_message
git add .
git commit -m "$commit_message"
git push