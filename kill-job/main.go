package main

import (
	"crypto/rand"
	"encoding/json"
	"io"
	"log"
	"math/big"
	"net/http"
	"os"
	"strconv"
	"time"
)

type Endpoint struct {
	url    string
	method string
}

var RUST_MAIN_APP_URL = "http://localhost:8084"
var GO_MAIN_APP_URL = "http://localhost:8085"
var MAX_REQUESTS = 5
var SERVICE_DISCOVERER_URL = "http://localhost:8086"

type AppService struct {
	url    string   `json:"url"`
	port   int32    `json:"port"`
	labels []string `json:"labels"`
	ip     string   `json:"ip"`
	name   string   `json:"name"`
}

func main() {
	log.Println("Application starting")
	loadEnvironmentVariables()
	for i := 0; i < MAX_REQUESTS; i++ {
		log.Println("Calling " + SERVICE_DISCOVERER_URL + "/services")
		resp, err := http.Get(SERVICE_DISCOVERER_URL + "/services")
		if err != nil {
			log.Println("ERROR calling SERVICE_DISCOVERER_URL: " + err.Error())
			return
		}
		defer resp.Body.Close()
		log.Println("Response status: " + resp.Status)
		body, err := io.ReadAll(resp.Body)
		if err != nil {
			log.Println("ERROR reading response body: " + err.Error())
			return
		}
		log.Println("Response body: " + string(body))
		var availableServices []AppService
		err = json.Unmarshal(body, &availableServices)
		if err != nil {
			log.Println("ERROR unmarshalling json: " + err.Error())
			return
		}
		for _, service := range availableServices {
			log.Println("Found service " + service.name + " at " + service.url)
		}

		random, _ := rand.Int(rand.Reader, big.NewInt(int64(len(availableServices))))
		randomEndpoint, randomMethod := getRandomEndpoint()
		makeRequest(Endpoint{
			url:    "http://" + availableServices[random.Int64()].url + "/" + randomEndpoint,
			method: randomMethod,
		})
		time.Sleep(500 * time.Millisecond)

	}
	log.Println("Application exiting")
}

func getRandomEndpoint() (string, string) {
	var endpoints = []string{"die", "hello", "not-existent"}
	random1, _ := rand.Int(rand.Reader, big.NewInt(int64(len(endpoints))))
	var methods = []string{http.MethodGet, http.MethodPost}
	random2, _ := rand.Int(rand.Reader, big.NewInt(int64(len(methods))))
	return endpoints[random1.Int64()], methods[random2.Int64()]
}

func loadEnvironmentVariables() {
	log.Println("Loading environment variables")
	if rustAppUrl := os.Getenv("RUST_MAIN_APP_URL"); rustAppUrl != "" {
		RUST_MAIN_APP_URL = rustAppUrl
		log.Println("RUST_MAIN_APP_URL set to " + RUST_MAIN_APP_URL + " via RUST_MAIN_APP_URL environment variable")
	}
	if goAppUrl := os.Getenv("GO_MAIN_APP_URL"); goAppUrl != "" {
		GO_MAIN_APP_URL = goAppUrl
		log.Println("GO_MAIN_APP_URL set to " + GO_MAIN_APP_URL + " via GO_MAIN_APP_URL environment variable")
	}
	if maxRequests := os.Getenv("MAX_REQUESTS"); maxRequests != "" {
		MAX_REQUESTS, _ = strconv.Atoi(maxRequests)
		log.Println("MAX_REQUESTS set to " + strconv.Itoa(MAX_REQUESTS) + " via MAX_REQUESTS environment variable")
	}
	if serviceDiscovererUrl := os.Getenv("SERVICE_DISCOVERER_URL"); serviceDiscovererUrl != "" {
		SERVICE_DISCOVERER_URL = serviceDiscovererUrl
		log.Println("SERVICE_DISCOVERER_URL set to " + SERVICE_DISCOVERER_URL + " via SERVICE_DISCOVERER_URL environment variable")
	}
}

func makeRequest(endpoint Endpoint) {
	log.Println("Calling " + endpoint.url + " via " + endpoint.method)
	if endpoint.method == http.MethodGet {
		resp, err := http.Get(endpoint.url)
		if err != nil {
			log.Println("ERROR: " + err.Error())
			return
		}
		log.Println("Response status: " + resp.Status)
		return
	} else if endpoint.method == http.MethodPost {
		resp, err := http.Post(endpoint.url, "text/plain", nil)
		if err != nil {
			log.Println("ERROR: " + err.Error())
			return
		}
		log.Println("Response status: " + resp.Status)
		return
	}
	log.Println("Method " + endpoint.method + " not supported")
}
