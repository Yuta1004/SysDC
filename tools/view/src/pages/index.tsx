import React from "react";

import Header from "../components/header";
import Flow from "../components/flow";

const App = () => (
    <div
        className="container"
        style={{
            width: "100vw",
            height: "100vh"
        }}
    >
        <Header/>
        <Flow
            onLoadStart={() => console.log("loadStart")}
            onLoadFinish={() => console.log("loadFinish")}
        />
    </div>
);

export default App;
