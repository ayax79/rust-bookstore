#!/bin/sh
kubectl logs $(kubectl get pods -l app=rust-bookstore-default -o name) -f -c rust-bookstore-default