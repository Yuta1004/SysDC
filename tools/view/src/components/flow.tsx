import React, { useEffect } from "react";
import ReactFlow, {
    Background,
    MiniMap,
    Controls,
    useNodesState,
    useEdgesState,
} from "react-flow-renderer";
import { invoke } from "@tauri-apps/api/tauri";

import layout from "./flow/layout";
import CUSTOM_NODE_TYPES from "./flow/custom";

type FlowProps = {
    onLoadStart?: () => void;
    onLoadFinish?: () => void;
}

export const Flow = (props: FlowProps) => {
    const [nodes, setNodes, onNodesChange] = useNodesState([]);
    const [edges, setEdges, onEdgesChange] = useEdgesState([]);

    useEffect(() => {
        props.onLoadStart();
        invoke("gen_flow").then(([nodes, edges]) => {
            layout(nodes, edges);
            Array.isArray(nodes) && setNodes(nodes);
            Array.isArray(edges) && setEdges(edges);
            props.onLoadFinish();
        });
    }, []);

    return (
        <ReactFlow
            nodes={nodes}
            edges={edges}
            onNodesChange={onNodesChange}
            onEdgesChange={onEdgesChange}
            nodeTypes={CUSTOM_NODE_TYPES}
            defaultEdgeOptions={{ zIndex: 9999 }}
            minZoom={0}
            fitView
        >
            <Background
                gap={24}
                size={1.5}
                color="#0006"
            />
            <MiniMap/>
            <Controls/>
        </ReactFlow>
    );
}

export default Flow;
