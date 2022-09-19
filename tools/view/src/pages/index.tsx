import React, { useEffect, useMemo } from "react";
import ReactFlow, { MiniMap, Controls, useNodesState, useEdgesState } from "react-flow-renderer";
import { invoke } from "@tauri-apps/api/tauri";

import layout from "../flow/layout";
import { UnitNode, ModuleNode, FunctionNode, VarNode } from "../flow/custom";

function App() {
    const [nodes, setNodes, onNodesChange] = useNodesState([]);
    const [edges, setEdges, onEdgesChange] = useEdgesState([]);

    const customNodeTypes = useMemo(() => ({
        Unit: UnitNode,
        Module: ModuleNode,
        Function: FunctionNode,
        Var: VarNode
    }), []);

    useEffect(() => {
        invoke("get_flow").then(([nodes, edges]) => {
            layout(nodes, edges);
            Array.isArray(nodes) && setNodes(nodes);
            Array.isArray(edges) && setEdges(edges);
        });
    }, []);

    return (
        <div
            className="container"
            style={{
                width: "100vw",
                height: "100vh"
            }}
        >
            <ReactFlow
                nodes={nodes}
                edges={edges}
                onNodesChange={onNodesChange}
                onEdgesChange={onEdgesChange}
                nodeTypes={customNodeTypes}
                defaultEdgeOptions={{ zIndex: 9999 }}
                fitView
            >
                <MiniMap/>
                <Controls/>
            </ReactFlow>
        </div>
    );
}

export default App;
