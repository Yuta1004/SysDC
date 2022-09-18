import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import ReactFlow from "react-flow-renderer";

import { convert } from "../sysdc_core/convert";

function App() {
    const [system, setSystem] = useState({ units: [] });

    const initialNodes = [
        {
            id: '1',
            type: 'input',
            data: { label: 'Input Node' },
            position: { x: 250, y: 25 },
        },

        {
            id: '2',
            // you can also pass a React component as a label
            data: { label: <div>Default Node</div> },
            position: { x: 100, y: 125 },
        },
        {
            id: '3',
            type: 'output',
            data: { label: 'Output Node' },
            position: { x: 250, y: 250 },
        },
    ];

    const initialEdges = [
        { id: 'e1-2', source: '1', target: '2' },
        { id: 'e2-3', source: '2', target: '3', animated: true },
    ];

    useEffect(() => {
        (async () => {
            await listen("initialize_system", event => {
                if (typeof event.payload == "object") {
                    setSystem(convert(event.payload));
                }
            });
        })();
    }, []);

    return (
        <div
            className="container"
            style={{
                width: "100vw",
                height: "100vw"
            }}
        >
            <h1>SysDC</h1>
            <p>{ JSON.stringify(system) }</p>
            <hr/>
            <ReactFlow
                nodes={initialNodes}
                edges={initialEdges}
                fitView
            />
        </div>
    );
}

export default App;
