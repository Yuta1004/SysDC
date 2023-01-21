import { useEffect, useState, createContext } from "react";

import Flow from "./components/flow";
import init from "sysdc_tool_view";

export const WasmContext = createContext(false);

const App = () => {
    const [wasmOk, setWasmOk] = useState(false);
    const [system, setSystem] = useState({ units: [] });

    window.addEventListener("message", (e: MessageEvent) => {
        setSystem(e.data)
    });

    useEffect(() => {
        init().then(() => setWasmOk(true));
    }, []);

    return (
        <div
            className="container"
            style={{
                width: "100vw",
                height: "100vh"
            }}
        >
            <WasmContext.Provider value={ wasmOk }>
                <Flow system={ system }/>
            </WasmContext.Provider>
        </div>
    );
}

export default App;
