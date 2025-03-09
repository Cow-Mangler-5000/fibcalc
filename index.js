import init, { fibonacci } from "./pkg/fibcalc.js";

const calculate = document.getElementById("calculate")
const input = document.getElementById("inp")
const result = document.getElementById("result")

init().then(() => {
    calculate.addEventListener("click", async () => {
        let x = input.value
        let y = Number(x)
        result.textContent = fibonacci(y);
    })
})