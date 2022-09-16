import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";

function App() {
    const [system, setSystem] = useState({});

    useEffect(() => {
        (async () => {
            await listen("initialize_system", event => {
                if (typeof event.payload == "object") {
                    setSystem(event.payload);
                }
            });
        })();
    }, []);

    return (
        <div className="container">
            <h1>SysDC</h1>
            <p>{ JSON.stringify(system) }</p>
        </div>
    );
}

export default App;
