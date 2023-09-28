#!/bin/bash

export ROCKET_ADDRESS=0.0.0.0
/opt/rabbit-locker/rabbit-locker &> /opt/rabbit-locker/rabbit-locker.log &

rabbitmq-server