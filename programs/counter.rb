## Given a list of numbers, build a dictionary
## with the count of each number
count_items : List[k] -> Dict[k, U64]
count_items = |items|
    update_count : Result[U64, [Missing]] -> Result[U64, [Missing]]
    update_count = |v| Ok(v.with_default(0).add(1))
    func = |d, v| d.update(v, update_count)
    items.walk(Dict.empty(), func)


