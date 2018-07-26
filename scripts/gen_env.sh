#!/bin/sh

REDIS_NAME=$(kubectl get pods -l app=redis -o name | grep master | sed -e 's/pod\///g' | sed -e 's/\-redis\-master\-[0-9]*//g')

cat > k8s/.env << EOF
# Redis environment info for forge
REDIS_HOST=${REDIS_NAME}-redis-master.default.svc.cluster.local
REDIS_SECRET=${REDIS_NAME}-redis
EOF