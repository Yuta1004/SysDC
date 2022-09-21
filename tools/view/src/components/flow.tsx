import React, { useEffect, useState } from "react";
import ReactFlow, {
    Background,
    MiniMap,
    Controls,
    useNodesState,
    useEdgesState,
} from "react-flow-renderer";
import CircularProgress from "@mui/material/CircularProgress";
import { invoke } from "@tauri-apps/api/tauri";

import layout from "./flow/layout";
import CUSTOM_NODE_TYPES from "./flow/custom";

export const Flow = () => {
    const [nowLoading, setNowLoading] = useState(true);
    const [nodes, setNodes, onNodesChange] = useNodesState([]);
    const [edges, setEdges, onEdgesChange] = useEdgesState([]);

    useEffect(() => {
        invoke("gen_flow").then(([nodes, edges]) => {
            layout(nodes, edges);
            Array.isArray(nodes) && setNodes(nodes);
            Array.isArray(edges) && setEdges(edges);
            setNowLoading(false);
        });
    }, []);

    return (<>
        <CircularProgress
            variant="indeterminate"
            disableShrink
            size={75}
            thickness={4}
            style={{
                display: nowLoading ? "block" : "none",
                position: "absolute",
                top: "50%",
                left: "50%",
                transform: "translate(-50%, -50%)",
                WebkitTransform: "translate(-50%, -50%)",
            }}
        />
        <ReactFlow
            nodes={nodes}
            edges={edges}
            onNodesChange={onNodesChange}
            onEdgesChange={onEdgesChange}
            nodeTypes={CUSTOM_NODE_TYPES}
            defaultEdgeOptions={{ zIndex: 9999 }}
            minZoom={0}
            fitView
            style={{
                visibility: nowLoading ? "hidden" : "visible"
            }}
        >
            <Background
                gap={24}
                size={1.5}
                color="#0006"
            />
            <MiniMap/>
            <Controls/>
        </ReactFlow>
    </>);
}

export default Flow;
