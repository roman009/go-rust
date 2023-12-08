package main

import (
	"crypto/rand"
	"log"
	"math/big"
	"net/http"
)

type Endpoint struct {
	url    string
	method string
}

func main() {
	log.Println("Application starting")
	var urls = []Endpoint{
		{url: "http://rust-main.default.svc.cluster.local:8084/hello", method: http.MethodGet},
		{url: "http://go-main.default.svc.cluster.local:8085/hello", method: http.MethodGet},
		{url: "http://rust-main.default.svc.cluster.local:8084/not-existent", method: http.MethodGet},
		{url: "http://go-main.default.svc.cluster.local:8085/not-existent", method: http.MethodGet},
		{url: "http://rust-main.default.svc.cluster.local:8084/die", method: http.MethodPost},
		{url: "http://go-main.default.svc.cluster.local:8085/die", method: http.MethodPost},
	}

	// select a random endpoint and perform a GET http request
	random, _ := rand.Int(rand.Reader, big.NewInt(int64(len(urls)))) // Convert len(urls) to *big.Int
	var endpoint = urls[random.Int64()]
	makeRequest(endpoint)
	log.Println("Application exiting")
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
