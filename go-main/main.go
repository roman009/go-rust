package main

import (
	"log"
	"net/http"
)

func main() {
	log.Println("Application starting")
	log.Println("Serving message " + getMessage() + " via HTTP on this endpoint: " + getEndpoint())
	http.HandleFunc("/hello", getHello)
	log.Fatal(http.ListenAndServe(":8085", nil))
}

func getHello(w http.ResponseWriter, r *http.Request) {
	log.Println("Connection established")
	w.Write([]byte(getMessage()))
}

func getMessage() string {
	return "Hello, world!"
}

func getServer() string {
	return "http://localhost:8085"
}

func getEndpoint() string {
	return getServer() + "/hello"
}
