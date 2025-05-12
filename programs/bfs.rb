module []

bfs : Dict[a, List[a]], a -> List[a]
bfs = |graph, start|
    traverse = |queue, visited|
        when queue is
            [] -> []
            [head, ..tail] ->
                neighbors =
                    graph.get(head)
                        .with_default([])
                        .drop_if(|n| visited.contains(n)))

                new_visited = neighbors.walk(visited, Set.insert)
                new_queue = tail.concat(neighbors)
                [head].concat(traverse(new_queue, new_visited))

    traverse([start], Set.single(start))

g1 =
    Dict.empty()
        .insert("a", ["b", "c", "e"])
        .insert("b", [])
        .insert("c", ["d", "e"])
        .insert("d", ["b"])
        .insert("e", [])

expect bfs(g1, "a") == ["a", "b", "c", "e", "d"]

g2 =
    Dict.empty()
        .insert("A", ["B", "C"])
        .insert("B", ["A", "D", "E"])
        .insert("C", ["A", "F"])
        .insert("D", ["B"])
        .insert("E", ["B", "F"])
        .insert("F", ["C", "E"])

expect bfs(g2, "C") == ["C", "A", "F", "B", "E", "D"]

