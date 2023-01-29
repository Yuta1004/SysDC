import "normalize.css";
import React from "react";
import ReactDOM from "react-dom/client";
import { Helmet } from "react-helmet";
import App from "./App";

const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
);
root.render(
  <React.StrictMode>
    <Helmet>
      <meta charset="utf-8" />
      <meta name="viewport" content="width=device-width, initial-scale=1" />
      <meta name="theme-color" content="#000000" />
      <meta
        property="og:url"
        content="%PUBLIC_URL%/sysdc-ogp.png"
      />
      <link rel="icon" href="%PUBLIC_URL%/favicon.png" />
      <title>SysDC Editor</title>
    </Helmet>
    <App />
  </React.StrictMode>
);
