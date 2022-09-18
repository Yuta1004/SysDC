import ReactFlow, { Node, Edge } from "react-flow-renderer";

export default FlowComponent;

export function FlowComponent({nodes, edges}) {
    return (
        <ReactFlow
            nodes={nodes}
            edges={edges}
            fitView
        />
    );
}
