import init, { Parameters, read_obj } from "./pkg/nanolabo_wasm.js";

function readFile(file) {
  return new Promise((resolve, reject) => {
    // Create file reader
    let reader = new FileReader();

    // Register event listeners
    reader.addEventListener("loadend", (e) => resolve(e.target.result));
    reader.addEventListener("error", reject);

    // Read file
    reader.readAsArrayBuffer(file);
  });
}

// https://localcoder.org/passing-client-files-to-webassembly-from-the-front-end
async function handleFile(file) {
  var parameters = new Parameters();
  parameters.export_format = 25;
  parameters.polygon_reduction = 0.5;

  // Todo: Stop copying in a JS buffer, instead copy direct to a wasm buffer
  // https://wasmbyexample.dev/examples/webassembly-linear-memory/webassembly-linear-memory.rust.en-us.html#
  var array = new Uint8Array(await readFile(file));
  console.log(array);
  var result = read_obj(parameters, array);
  console.log("Output size: " + result.length);

  // Download result back
  var blob = new Blob([result], { type: "application/pdf" }); // change resultByte to bytes
  var link = document.createElement("a");
  link.href = window.URL.createObjectURL(blob);
  link.download = "output.obj";
  link.click();
}

// Initialize wasm module
init();

// Selecting all required elements
const dropArea = document.getElementById("drag-area");
const dragText = dropArea.querySelector("header");
const button = dropArea.querySelector("button");
const input = dropArea.querySelector("input");

button.onclick = () => {
  input.click(); // If user click on the button then the input also clicked
};

input.onchange = function () {
  // Getting user select file and [0] this means if user select multiple files then we'll select only the first one
  dropArea.classList.add("active");
  showFile(this.files);
};

// If user Drag File Over DropArea
dropArea.ondragover = function (event) {
  event.preventDefault(); // Preventing from default behaviour
  dropArea.classList.add("active");
  dragText.textContent = "Release to Upload File";
};

// If user leave dragged File from DropArea
dropArea.ondragleave = function () {
  dropArea.classList.remove("active");
  dragText.textContent = "Drag & Drop to Upload File";
};

//If user drop File on DropArea
dropArea.ondrop = function (event) {
  event.preventDefault(); //preventing from default behaviour
  //getting user select file and [0] this means if user select multiple files then we'll select only the first one
  showFile(event.dataTransfer.files); //calling function
};

function showFile(files) {
  let validExtensions = ["obj", "step", "stp"]; // Adding some valid image extensions in array

  for (var i = 0; i < files.length; i++) {
    console.log(files[i]);
    let format = files[i].name.split(".").pop().toLowerCase();
    if (!validExtensions.includes(format)) {
      alert(`"${format}" is not a supported file format!`);
      dropArea.classList.remove("active");
      dragText.textContent = "Drag & Drop to Upload File";
      return;
    }
  }

  for (var i = 0; i < files.length; i++) {
    handleFile(files[i]);
  }
}