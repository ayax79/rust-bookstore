#!/bin/sh

HOST=$(minikube service --url rust-bookstore-default)
curl -X POST -H 'Content-Type: application/json' -i "$HOST/book/" --data '{
"book_id": "0bcd291d-b7c5-4390-965f-8a70707d22a5",
"author": "Robert Jordan",
"title": "Eye of the World"
}'