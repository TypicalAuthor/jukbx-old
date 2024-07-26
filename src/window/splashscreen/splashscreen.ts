import "../../styles.css";
import App from "./splashscreen.svelte";

const app = new App({
    target: document.getElementById("app")!,
});

export default app;