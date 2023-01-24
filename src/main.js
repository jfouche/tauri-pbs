const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;
let inputPartNumer;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

class NewItemController {
  constructor() {
    console.log("NewItemController::constructor()");
    this.inputNewItemPartNumber = document.getElementById("input-pn");
    this.inputNewItemName = document.getElementById("input-name");

    document
      .getElementById("btn-new-item")
      .addEventListener("click", () => this.submit());
  }

  submit() {
    let newItem = {
      pn: this.inputNewItemPartNumber.value,
      name: this.inputNewItemName.value

    };

    invoke("new_part_number", { newItem }).then((msg) => {
      console.log("OK : ", msg);
    }).catch((err) => {
      console.error("Erreur : ", err);
    })
  }
}

let newItemController;

window.addEventListener("DOMContentLoaded", () => {
  // greetInputEl = document.querySelector("#greet-input");
  // greetMsgEl = document.querySelector("#greet-msg");
  // inputPartNumer = document.getElementById("input-pn");

  // document
  //   .querySelector("#greet-button")
  //   .addEventListener("click", () => greet());

  newItemController = new NewItemController();
});
