import init, { process_image } from "/docs/pkg/pxlate_web.js";

await init();
document.getElementById("process-btn").onclick = async () => {
  try {
    const fileInput = document.getElementById("image-input");
    const density = parseInt(document.getElementById("density").value);

    if (!fileInput.files.length) return alert("Choose an image first!");
    const file = fileInput.files[0];
    const arrayBuffer = await file.arrayBuffer();
    const uint8Array = new Uint8Array(arrayBuffer);

    const resultDataUrl = await process_image(uint8Array, density);

    const img = document.getElementById("output-image");
    img.src = resultDataUrl;
    img.style.display = "block";

    const link = document.getElementById("download-link");
    link.href = resultDataUrl;
    link.style.display = "inline-block";
  } catch (err) {
    alert("Failed to generate image: " + err);
    console.error(err);
  }
};
