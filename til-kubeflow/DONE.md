# Done

- `git clone git@github.com:kubeflow/manifests.git`
- fix `env` to `envs` in kustomization.yaml in jupyter-web-app
- `kubectl create namespace kubeflow`
- `cd manifests`
- `kustomize build jupyter/jupyter-web-app/base/ | kubectl create -f`
- `kubectl create namespace istio-system`
- fix `env` to `envs` in kustomization.yaml in ambassador
- change namespace from `istio-system` to `kubeflow` in kustomization.yaml in ambassador
- `kustomize build common/ambassador/base/ | kubectl create -f -`
- fix `env` to `envs` in kustomization.yaml in notebook-controller
- `kustomize build jupyter/notebook-controller/base/ | kubectl create -f -`
- `kustomize build kubeflow-roles/base/ | kubectl create -f -`
