package main

import (
	"log"
	"net/http"
	"os"
	"strconv"

	"github.com/prometheus/client_golang/prometheus/promhttp"
)

var PORT = 8085

func main() {
	log.Println("Application starting")
	loadEnvironmentVariables()
	log.Println("Serving message " + getMessage() + " via HTTP on this endpoint: " + getEndpoint())
	http.HandleFunc("/hello", getHello)
	http.HandleFunc("/die", postDie)
	http.HandleFunc("/health", getHealth)
	http.HandleFunc("/metrics", promhttp.Handler())
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
	w.WriteHeader(http.StatusOK)
	w.(http.Flusher).Flush()
	log.Println("Application exiting")
	os.Exit(0)
}

func getHello(w http.ResponseWriter, r *http.Request) {
	log.Println("Connection established")
	w.WriteHeader(http.StatusOK)
	w.Write([]byte(getMessage()))
}

func getHealth(w http.ResponseWriter, r *http.Request) {
	log.Println("Health check connection established")
	w.WriteHeader(http.StatusOK)
	w.Write([]byte("OK"))
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
