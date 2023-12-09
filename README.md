# go-rust

One Rust and one golang microservice that are deployed to microk8s. There is a cron job defined on the cluster that runs every minute and calls a random enpoint on a microservice. Each microservice has an endpoint `/die` that will cause the pod to die forcing the HorizontalPodAutoscaler to create a new pod.

## microk8s

```bash
microk8s kubectl describe secret -n kube-system microk8s-dashboard-token
```

```bash
./build-and-deploy.sh
```
