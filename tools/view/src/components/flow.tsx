import ReactFlow, { Node, Edge } from "react-flow-renderer";
import dagre from "dagre";

export default FlowComponent;

export function FlowComponent({nodes, edges}) {
    const nodesWithPos = autoLayout(nodes, edges);
    return (
        <ReactFlow
            nodes={nodesWithPos}
            edges={edges}
            fitView
        />
    );
}

function autoLayout(nodes: Node<any>[], edges: Edge<any>[]): Node<any>[] {
    const dagreGraph = new dagre.graphlib.Graph();
    dagreGraph.setGraph({ rankdir: "TB" });
    dagreGraph.setDefaultEdgeLabel(() => ({}));

    nodes.forEach(node => {
        dagreGraph.setNode(node.id, { width: 200, height: 50 });
    });
    edges.forEach(edge => {
        dagreGraph.setEdge(edge.source, edge.target);
    });

    dagre.layout(dagreGraph);

    return nodes.map(node => {
        const nodeWithPos = dagreGraph.node(node.id);
        node.position = {
            x: nodeWithPos.x - 100,
            y: nodeWithPos.y - 25
        };
        return node;
    });
}
