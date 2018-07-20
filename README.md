# rust-bookstore

A sample app I use for playing around with rust microservices.

This micro service currently will accept the following calls:

* GET /bookstore/{upmID} - Retrieve a book
* POST /bookstore/ - Create a book
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
    - _helm install stable/redis > /tmp/install_notes.txt_
* Configure Deployment (todo - make less sucky)
    - _forge setup_
    - Modify deployment.yaml's BOOKSTORE_REDISHOST BOOKSTORE_REDISPASSWORD to match the redis install names (it will be different per install)
* Deploy
    _forge deploy_
    _kubectl get pods,services_

