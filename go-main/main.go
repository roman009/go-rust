package main

import (
	"log"
	"net/http"
	"os"
	"strconv"
)

var PORT = 8085

func main() {
	log.Println("Application starting")
	loadEnvironmentVariables()
	log.Println("Serving message " + getMessage() + " via HTTP on this endpoint: " + getEndpoint())
	http.HandleFunc("/hello", getHello)
	http.HandleFunc("/die", postDie)
	log.Fatal(http.ListenAndServe(":"+strconv.Itoa(PORT), nil))
}

func loadEnvironmentVariables() {
	log.Println("Loading environment variables")
	if port := os.Getenv("LISTENING_PORT"); port != "" {
		var err error
		PORT, err = strconv.Atoi(port)
		if err != nil {
			log.Fatal(err)
		}
		log.Println("PORT set to " + port + " via LISTENING_PORT environment variable")
	}
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
	return "http://0.0.0.0:" + strconv.Itoa(PORT)
}

func getEndpoint() string {
	return getServer() + "/hello"
}
