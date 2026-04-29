let lib = import "lib.ncl",
in
let JobOptions = {
  image | String,
  args | Array String,
}
in
{
  config = {
    k8s-jobs
      | { _ | { _ | JobOptions } }
      = {},

    k8s-resources =
      k8s-jobs
      |> std.record.map (fun jobNamespace jobs =>
        jobs
        |> std.record.map (fun jobName jobOptions =>
          {
            apiVersion = "batch/v1",
            kind = "Job",
            metadata.name = jobName,
            metadata.namespace = jobNamespace,
            spec.template.spec.containers = [
              {
                name = jobName,
                image = jobOptions.image,
                args = jobOptions.args,
              }
            ],
          }
        )
      )
      |> lib.record.flatten (fun jobNamespace jobName => "jobs/%{jobNamespace}/%{jobName}.yaml")
  },
}
