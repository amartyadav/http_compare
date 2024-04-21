import { invoke } from "@tauri-apps/api/tauri";
import { open } from '@tauri-apps/api/dialog';

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

async function openInputFileSelector() {
  const selected = await open({
    multiple: false,
    filters: [{
      name: "JSON",
      extensions: ["json"]
    }]
  });
  console.group("Selected files");
  console.log(selected);
  console.groupEnd();
}

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#select-file1")?.addEventListener("click", openInputFileSelector);
  document.querySelector("#select-file2")?.addEventListener("click", openInputFileSelector);
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
