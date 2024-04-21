export function loadTyping(el: HTMLDivElement) {
  if (!el.classList.contains("typing")) el.classList.add("typing");
  const len = (el.textContent ?? "").length + 1;
  el.style.animation = `typing ${Math.min(
    1,
    0.1 * len,
  )}s steps(${len}), blink .4s step-end infinite alternate`;
  el.style.width = `${len}ch`;
  el.addEventListener("animationend", () => {
    el.style.animation = "none";
    el.style.borderRight = "none";
  });
}
