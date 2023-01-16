import { useEffect, useState, createContext } from "react";

import init from "sysdc_tool_check";

export const WasmContext = createContext(false);

const App = () => {
    const [wasmOk, setWasmOk] = useState(false);
    const [system, setSystem] = useState("{}");

    const entrypoint = (event: MessageEvent) => {
        setSystem(event.data);
    };
    window.addEventListener("message", entrypoint);

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
                WasmStat : { wasmOk }<br/>
                System : { system }<br/>
            </WasmContext.Provider>
        </div>
    );
}

export default App;
