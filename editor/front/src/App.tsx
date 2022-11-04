import Header from "./components/Header";
import Editor from "./components/Editor";

const App = () => {
    return (
        <div
            style={{
                display: "flex",
                flexDirection: "column",
                width: "100vw",
                height: "100vh"
            }} 
        >
            <Header
                style={{
                    flex: 1
                }}
            />
            <Editor
                style={{
                    width: "100%",
                    height: "100%"
                }} 
            />
        </div>
    )
};

export default App;
