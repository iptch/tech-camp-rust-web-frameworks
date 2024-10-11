# ho-erfa-sample

## Sample Architecture

We will build a containerized Rust application with Tokio. This application will be fully stateless
in order to allow horizontal scaling. Rust should provide us with enough performance and stability
for a productive system. We will have a persistency layer with MongoDB and a caching system with
Redis.

The whole thing will be deployed via Helm on a generic Kubernetes platform. This way we have only
the costs of the platform and have close to no lock-in.

We will not build any CD, but this would be done with ArgoCD easily since we work with Helm.

CI will also not be updated, but general testing will be integrated into the docker build process
and can thus be ported to any docker compatible CI tool that is desired.

## Cluster Setup

To deploy to a sample cluster use the following:

```sh
# setup cluster
k3d cluster create erfa -s 1 -a 2 --api-port 0.0.0.0:6550 -p "8080:80@loadbalancer" --registry-create "registry.localhost:registry.localhost:5000"
kubectl create ns jbe-ho-erfa

# build image
docker build -t registry.localhost:5000/jbe/ho-erfa:0.1.0 .
docker push registry.localhost:5000/jbe/ho-erfa:0.1.0

# deploy a sharded HA mongo, this can take some time
helm repo add bitnami https://charts.bitnami.com/bitnami
helm install -n jbe-ho-erfa mongo bitnami/mongodb-sharded --set 'auth.rootPassword=jbe,mongos.replicaCount=2,shardsvr.dataNode.replicaCount=2,configsvr.replicaCount=2'
helm install -n jbe-ho-erfa myapp ./chart/
```
