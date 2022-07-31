import { Event as TauriEvent, listen } from "@tauri-apps/api/event";

const main = () => {
   const displayEL = document.getElementById("cpu-monitor");
   listen("usage", (event: TauriEvent<String>) => {
    displayEL!.innerHTML = JSON.stringify(event.payload);
   });
}

main();