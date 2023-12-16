# go-rust

One Rust and one golang microservice that are deployed to microk8s. There is a cron job defined on the cluster that runs every minute and calls a random enpoint on a microservice. Each microservice has an endpoint `/die` that will cause the pod to die forcing the HorizontalPodAutoscaler to create a new pod.

## prometheus methics

Each microservice has a `/metrics` endpoint that is scraped by prometheus. The prometheus server is deployed to the cluster and is configured to scrape the microservices.


## microk8s

```bash
microk8s kubectl describe secret -n kube-system microk8s-dashboard-token
```

```bash
./build-and-deploy.sh
```

## todo

1. publish metrics of servive kill calls (prometheus?/kafka?)

2. create a reactive be4fe(?) that handles websocket connections to push messages to a UI (react?) that shows when a service is killed