{
  config = {
    k8s-namespaces
      | { _ | {} }
      = {},

    k8s-resources =
      k8s-namespaces
      |> std.record.to_array
      |> std.array.map (fun { field = namespace, value = _ } =>
        {
          field = "namespaces/%{namespace}.yaml",
          value = {
            apiVersion = "v1",
            kind = "Namespace",
            metadata.name = namespace,
          },
        }
      )
      |> std.record.from_array,
  },
}
