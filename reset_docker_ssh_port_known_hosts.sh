#!/bin/bash

source .env
cat ~/.ssh/known_hosts | grep $SSH_PORT
ssh-keygen -R "[localhost]$SSH_PORT"
ssh-keygen -R "[127.0.0.1]:$SSH_PORT"
cat ~/.ssh/known_hosts | grep $SSH_PORT

