import type { AppProps } from "next/app";

import "normalize.css";

export default function MyApp({ Component, pageProps }: AppProps) {
    return <Component {...pageProps} />;
}
