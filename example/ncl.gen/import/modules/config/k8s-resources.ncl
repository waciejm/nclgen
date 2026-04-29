{
  config = {
    k8s-resources
      | { _ | { .. } }
      | default
      = {},

    outputs = k8s-resources |> std.record.map_values (std.serialize 'Yaml),
  },
}
# keep-sorted start
& (import "outputs.ncl")
# keep-sorted end
