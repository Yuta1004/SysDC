import { useEffect, useContext } from "react";
import ReactFlow, {
    Background,
    MiniMap,
    Controls,
    useNodesState,
    useEdgesState,
} from "react-flow-renderer";

import { WasmContext } from "../App";
import layout from "./flow/layout";
import CUSTOM_NODE_TYPES from "./flow/custom";
import { gen_flow } from "sysdc_tool_view";

interface FlowProps {
    system: {}
}

export const Flow = (props: FlowProps) => {
    const wasmOk = useContext(WasmContext);

    const [nodes, setNodes, onNodesChange] = useNodesState([]);
    const [edges, setEdges, onEdgesChange] = useEdgesState([]);

    useEffect(() => {
        if (wasmOk) {
            const [nodes, edges] = gen_flow(props.system);
            layout(nodes, edges);
            Array.isArray(nodes) && setNodes(nodes);
            Array.isArray(edges) && setEdges(edges);
        }
    }, [props.system]);
 
    return (
        <ReactFlow
            nodes={ nodes }
            edges={ edges }
            onNodesChange={ onNodesChange }
            onEdgesChange={ onEdgesChange }
            nodeTypes={ CUSTOM_NODE_TYPES }
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
