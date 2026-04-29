{
  config
    | {
      # keep-sorted start
      k8s-jobs,
      k8s-namespaces,
      k8s-resources,
      outputs,
      # keep-sorted end
    },
}
# keep-sorted start
& (import "config/k8s-jobs.ncl")
& (import "config/k8s-namespaces.ncl")
& (import "config/k8s-resources.ncl")
& (import "config/outputs.ncl")
# keep-sorted end
