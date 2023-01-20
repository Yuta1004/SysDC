window.onmessage = (event) => App(event.data);

const App = (system: string) => {
    const view = document.getElementById("view");
    if (view !== null) {
        view.innerText = JSON.stringify(system);
    }
}
