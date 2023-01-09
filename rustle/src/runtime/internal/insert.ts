export function insert(target: Node, node: Node, anchor?: Node) {
    target.insertBefore(node, anchor || null);
}
