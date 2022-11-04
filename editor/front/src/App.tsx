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
            <Header/>
            <Editor/>
        </div>
    )
};

export default App;
