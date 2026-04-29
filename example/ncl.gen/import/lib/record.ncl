{
  map_fields
    : forall a. (String -> String) -> { _ : a } -> { _ : a }
    = (fun map_fn record =>
      record
      |> std.record.to_array
      |> std.array.map (fun { field = field_to_map, value } =>
        {
          field = map_fn field_to_map,
          include value,
        }
      )
      |> std.record.from_array
    ),

  flatten
    | forall a. (String -> String -> String) -> { _ : { _ : a } } -> { _ : a }
    = (fun map_fn record =>
      record
      |> std.record.map (fun nameOuter inner =>
        inner |> map_fields (map_fn nameOuter)
      )
      |> std.record.values
      |> std.record.merge_all
    ),
}
