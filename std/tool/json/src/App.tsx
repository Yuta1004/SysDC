import { useState } from "react";

import JSONPretty from "react-json-pretty";

const App = () => {
    const [system, setSystem] = useState<JSON>();

    const entrypoint = (event: MessageEvent) => {
        setSystem(JSON.parse(event.data));
    };
    window.addEventListener("message", entrypoint);

    return <JSONPretty data={ system }/>;
}

export default App;
