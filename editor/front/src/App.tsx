import Header from "./components/Header";
import FileExplorer from "./components/FileExplorer";
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
            <div
                style={{
                    display: "flex",
                    flexDirection: "row",
                    width: "100%",
                    height: "100%"
                }}
            >
                <FileExplorer
                    style={{
                        padding: 0,
                        flex: 1,
                        minWidth: "10%"
                    }} 
                />
                <Editor
                    style={{
                        width: "100%",
                        height: "100%"
                    }} 
                />
            </div>
        </div>
    )
};

export default App;
