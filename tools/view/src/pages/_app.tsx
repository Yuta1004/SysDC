import type { AppProps } from "next/app";

import "../style.css";
import "../App.css";

export default function MyApp({ Component, pageProps }: AppProps) {
    return <Component {...pageProps} />;
}
