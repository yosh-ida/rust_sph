// import("../pkg/index.js").catch(console.error);
(async () => {
    let wasm = await import("../pkg/index.js").catch(console.error);
    let wasm_bg = await import("../pkg/index_bg.wasm").catch(console.error);

    // here is sample codes...
    // const ptr = wasm.published_get_pointer_function();
    // const pixels = new Uint8Array(wasm_bg.memory.buffer, ptr, 1920 * 1080 * 4);

    let handler = {};
    let before = 0;
    let loop = (time) => {
        const now = time * 0.001;
        if (before === 0) {
            before = now;
            wasm_bg.draw(0.0, 0.0, 0.0, 0.0);
        } else {
            const dt = now - before;
            before = now;
            // wasm_bg.draw(0.0, 0.0, 0.0, dt);
            wasm_bg.draw(0.0, 0.0, 0.0, 0.01);
        }
        handler.id = requestAnimationFrame(loop);
    }
    handler.id = requestAnimationFrame(loop);
    // ループをキャンセル
    // cancelAnimationFrame(handler.id);
})();