window.onmessage = (event) => app(event.data);

const app = (system: string) => {
    const view = document.getElementById("view");
    if (view !== null) {
        view.innerText = system;
    }
}
