import AceEditor from "react-ace";
import "react-ace-builds/webpack-resolver-min";

const App = () => {
    return (
        <div>
            <AceEditor
                mode="python"
                theme="monokai"
                showPrintMargin={false}
                showGutter={true}
                highlightActiveLine={true}
                style={{
                    width: "100vw",
                    height: "100vh"
                }} 
            />
        </div>
    )
};

export default App;
