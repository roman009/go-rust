package main

import (
	"crypto/rand"
	"log"
	"math/big"
	"net/http"
	"os"
)

type Endpoint struct {
	url    string
	method string
}

var RUST_MAIN_APP_URL = "http://localhost:8084"
var GO_MAIN_APP_URL = "http://localhost:8085"

func main() {
	log.Println("Application starting")
	loadEnv()
	var urls = []Endpoint{
		{url: RUST_MAIN_APP_URL + "/hello", method: http.MethodGet},
		{url: GO_MAIN_APP_URL + "/hello", method: http.MethodGet},
		{url: RUST_MAIN_APP_URL + "/not-existent", method: http.MethodGet},
		{url: GO_MAIN_APP_URL + "/not-existent", method: http.MethodGet},
		{url: RUST_MAIN_APP_URL + "/die", method: http.MethodPost},
		{url: GO_MAIN_APP_URL + "/die", method: http.MethodPost},
	}
	random, _ := rand.Int(rand.Reader, big.NewInt(int64(len(urls))))
	var endpoint = urls[random.Int64()]
	makeRequest(endpoint)
	log.Println("Application exiting")
}

func loadEnv() {
	log.Println("Loading environment variables")
	if rustAppUrl := os.Getenv("RUST_MAIN_APP_URL"); rustAppUrl != "" {
		RUST_MAIN_APP_URL = rustAppUrl
		log.Println("RUST_MAIN_APP_URL set to " + rustAppUrl + " via RUST_MAIN_APP_URL environment variable")
	}
	if goAppUrl := os.Getenv("GO_MAIN_APP_URL"); goAppUrl != "" {
		GO_MAIN_APP_URL = goAppUrl
		log.Println("GO_MAIN_APP_URL set to " + goAppUrl + " via GO_MAIN_APP_URL environment variable")
	}
}

func makeRequest(endpoint Endpoint) {
	log.Println("Calling " + endpoint.url + " via " + endpoint.method)
	if endpoint.method == http.MethodGet {
		resp, err := http.Get(endpoint.url)
		if err != nil {
			log.Fatal(err)
		}
		log.Println("Response status: " + resp.Status)
		return
	} else if endpoint.method == http.MethodPost {
		resp, err := http.Post(endpoint.url, "text/plain", nil)
		if err != nil {
			log.Fatal(err)
		}
		log.Println("Response status: " + resp.Status)
		return
	}
	log.Fatal("Method not supported")
}
