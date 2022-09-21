import React, { useState } from "react";

import Header from "../components/header";
import Flow from "../components/flow";

const App = () => {
    const [nowLoading, setNowLoading] = useState(true); 

    return (
        <div
            className="container"
            style={{
                width: "100vw",
                height: "100vh"
            }}
        >
            <Header/>
            <Flow/>
        </div>
    );
}

export default App;
