import init, { run_app } from "../../pkg/week_parity_wasm.js";

async function main() {
	await init();
	await run_app();
}

main().catch(console.error);
