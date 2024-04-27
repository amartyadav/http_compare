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
    multiple: true,
    filters: [{
      name: "JSON",
      extensions: ["json"]
    }]
  });
  // add an error message if no file was selected
  if (selected === null || selected.length === 0) {
    console.error("No file selected");
    return;
  }

  // add an error message if more than 2 files were selected
  if (selected.length > 2) {
    console.error("Please select only 2 files");
    return;
  }

  console.group("Selected files");
  console.log(selected);
  console.groupEnd();

  let result = await invoke("read_and_parse_json", {
    jsonFilePaths: selected
  });

  console.group("Result");
  console.log(result);
  console.groupEnd();
}

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#select-files")?.addEventListener("click", openInputFileSelector);
  // document.querySelector("#select-file2")?.addEventListener("click", openInputFileSelector);
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
