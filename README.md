# KubExplorer
[![Rust build & tests](https://github.com/Pscheidl/kubexplorer/actions/workflows/rust.yml/badge.svg)](https://github.com/Pscheidl/kubexplorer/actions/workflows/rust.yml)

**Warning:** Proof of concept. Feedback is much welcome.

Discovers and prints out any `Configmaps` and `Secrets` not linked to any of the following resources:
1. Deployments,
1. ReplicaSets,
1. StatefulSets,
1. DaemonSets,
1. Jobs,
1. CronJobs,
1. ReplicationControllers,
1. Pods.

## Running

1. [Install Rust](https://www.rust-lang.org/learn/get-started)
1. Simply invoke `cargo run -- -h` (add the `--release` flag for optimal performance) to obtain instructions.

`> cargo run -- -h`

```shell
KubEx - Kubernetes Explorer 0.1.0
Pavel Pscheidl <pavelpscheidl@gmail.com>
Discovers unused ConfigMaps and Secrets

USAGE:
    kubex [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -k, --kubeconfig <PATH_TO_KUBECONFIG>    Path to a KUBECONFIG file. When not set, env is used.
    -n, --namespace <NAMESPACE>              Namespace to search in.
```

E.g. `cargo run -- -k /etc/rancher/k3s/k3s.yaml -n default` to explicitly specify the `KUBECONFIG` and the namespace.
If `KUBECONFIG` is not specified, the `KUBECONFIG` env variable is looked for. When not found, an error is thrown.
If `namespace` is not defined, the default namespace from `KUBECONFIG` is used.

The tool will detect orphans in the `KUBECONFIG`'s default namespace.

## Testing

Run tests using `cargo test`. Tests require:

1. Running Kubernetes cluster with supported API version `1_19`,
1. `KUBECONFIG` environment variable set.

An easy way to obtain a Kubernetes cluster is [k3s.io](https://k3s.io/) - curl -sfL https://get.k3s.io | sh -. After
installation, `export KUBECONFIG=/etc/rancher/k3s/k3s.yaml` and make sure to `chown` or `chmod` the `$KUBECONFIG` file
for current user to be able to read it.