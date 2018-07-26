# rust-bookstore

A sample app I use for playing around with rust microservices.

This micro service currently will accept the following calls:

* GET /book/{upmID} - Retrieve a book
* POST /book/ - Create a book
example json:
{
    "book_id": "2c8a4ac9-65f5-42a3-9387-019fad35490c",
    "author": "Ernest Hemmingway",
    "title": "For Whom the Bell Tolls"
}

## Local Setup
* Install Docker (http://docker.io)
* Install kubectl (https://kubernetes.io/docs/tasks/tools/install-kubectl/)
* Install HyperKit (https://github.com/moby/hyperkit)
* Install Minikube (https://github.com/kubernetes/minikube)
    - _minikube start --vm-driver hyperkit_
* Install Helm (https://www.helm.sh/)
* Install Forge (https://forge.sh/)
* Install Redis
    - execute _helm install stable/redis > /tmp/install_notes.txt_
* Configure Forge
    - Configure forge. execute: _forge setup_
    - Configure to point at our redis install. execute: _scripts/gen_env.sh_
* Deploy
    _forge deploy_
    _kubectl get pods,services_

## Useful scripts
* Create an initial entry: _/scripts/post_book.sh_
* Retrieve the initial entry: _/scripts/get_book.sh_
* Generates redis config for forge based on the current redis install:  _/scripts/gen_env.sh_
* Tails logs: _scripts/tail_book.sh_

