package main

import (
	"log"
	"net/http"
)

func main() {
	log.Println("Application starting")
	log.Println("Serving message " + getMessage() + " via HTTP on this endpoint: " + getEndpoint())
	http.HandleFunc("/hello", getHello)
	http.HandleFunc("/die", postDie)
	log.Fatal(http.ListenAndServe(":8085", nil))
}

func postDie(w http.ResponseWriter, r *http.Request) {
	if r.Method != "POST" {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}
	log.Println("Connection established")
	w.Write([]byte("Goodbye, world!"))
	w.(http.Flusher).Flush()
	log.Fatal("Application exiting")
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
