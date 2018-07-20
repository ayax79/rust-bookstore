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