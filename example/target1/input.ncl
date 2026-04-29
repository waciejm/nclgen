let lib = import "lib.ncl"
in
(import "modules.ncl")
& {
  config = {
    k8s-namespaces = {
      one = {},
      two = {},
    },

    k8s-jobs = {
      one.hello = {
        image = "bash",
        args = ["-c", "hello from namespace one"],
      },
      two.hello = {
        image = "bash",
        args = ["-c", "hello from namespace two"],
      },
    },
  },
}
