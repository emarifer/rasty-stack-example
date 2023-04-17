/* @refresh reload */
import { render } from "solid-js/web";

import "./styles.css";
import "sweetalert2/dist/sweetalert2.min.css";
import App from "./App";

render(() => <App />, document.getElementById("root") as HTMLElement);
