import ReactFlow, { Node, Edge } from "react-flow-renderer";

function FlowComponent({nodes, edges}) {
    return (
        <ReactFlow
            nodes={nodes}
            edges={edges}
            fitView
        />
    );
}

export default FlowComponent;
