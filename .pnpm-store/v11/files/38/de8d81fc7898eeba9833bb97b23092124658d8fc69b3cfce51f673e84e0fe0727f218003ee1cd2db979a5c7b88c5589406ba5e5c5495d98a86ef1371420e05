import { DEVTOOLS_MOUNT_PATH } from "@vitejs/devtools-kit/constants";
import { defineNuxtPlugin } from "#imports";
export default defineNuxtPlugin(() => {
  const script = document.createElement("script");
  script.type = "module";
  script.src = `${DEVTOOLS_MOUNT_PATH}embedded.js`;
  document.body.appendChild(script);
});
