# go-rust

## build

```bash
docker build -t go-main:latest -f go-main.Dockerfile .
```

```bash
docker build -t rust-main:latest -f rust-main.Dockerfile .``

## microk8s

```bash
microk8s kubectl describe secret -n kube-system microk8s-dashboard-token
```

```bash