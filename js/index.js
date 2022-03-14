// import("../pkg/index.js").catch(console.error);
(async () => {
    let wasm = await import("../pkg/index.js").catch(console.error);
    let wasm_bg = await import("../pkg/index_bg.wasm").catch(console.error);

    let s_fps = document.getElementById('slider_fps');
    let s_eta = document.getElementById('slider_eta');
    let s_rho = document.getElementById('slider_rho');
    let s_p = document.getElementById('slider_p');
    let s_r = document.getElementById('slider_r');
    let s_h = document.getElementById('slider_h');

    let input_fps = document.getElementById('input_fps');
    let input_eta = document.getElementById('input_eta');
    let input_rho = document.getElementById('input_rho');
    let input_p = document.getElementById('input_p');
    let input_r = document.getElementById('input_r');
    let input_h = document.getElementById('input_h');

    s_fps.value = input_fps.value;
    s_eta.value = input_eta.value;
    s_rho.value = input_rho.value;
    s_p.value = input_p.value;
    s_r.value = input_r.value;
    s_h.value = input_h.value;

    let b_start = document.getElementById('button_start');
    let b_stop = document.getElementById('button_stop');

    let stop = true;
    let fps = 60;

    let handler = {};
    let before = 0;
    let loop = (time /* mili seconds */) => {
        if (before === 0) {
            before = time;
            wasm_bg.draw(0.0, 0.0, 0.0, 0.0);
        } else if (time - before >= 1000 / fps) {
            wasm_bg.draw(0.0, 0.0, 0.0, 1000 / fps);
            before = time;
        }
        handler.id = requestAnimationFrame(loop);
    }

    b_start.addEventListener(
        'click', () => {
            stop = false;
            before = 0;
            console.log("r: %f\nh: %f\nrho: %f\np: %f", s_r.value, s_h.value, s_rho.value, s_p.value);
            wasm_bg.init(s_r.value, s_h.value, s_rho.value, s_p.value);
            handler.id = requestAnimationFrame(loop);
        }, false
    );

    b_stop.addEventListener(
        'click', () => { if (!stop) cancelAnimationFrame(handler.id); stop = true; before = 0; }, false
    );

    s_fps.addEventListener(
        'input', (e) => { fps = e.target.value; input_fps.value = e.target.value; }, false
    );
    s_eta.addEventListener(
        'input', (e) => { input_eta.value = e.target.value }, false
    );
    s_rho.addEventListener(
        'input', (e) => { input_rho.value = e.target.value }, false
    );
    s_p.addEventListener(
        'input', (e) => { input_p.value = e.target.value }, false
    );
    s_r.addEventListener(
        'input', (e) => { input_r.value = e.target.value }, false
    );
    s_h.addEventListener(
        'input', (e) => { input_h.value = e.target.value }, false
    );

    input_fps.addEventListener(
        'input', (e) => {
            const v = Math.min(e.target.max, Math.max(e.target.min, e.target.value));
            s_fps.value = v;
            if (v !== e.target.value)
                e.target.value = v;
        }, false
    );
    input_eta.addEventListener(
        'input', (e) => {
            const v = Math.min(e.target.max, Math.max(e.target.min, e.target.value));
            s_eta.value = v;
            if (v !== e.target.value)
                e.target.value = v;
        }, false
    );
    input_rho.addEventListener(
        'input', (e) => {
            const v = Math.min(e.target.max, Math.max(e.target.min, e.target.value));
            s_rho.value = v;
            if (v !== e.target.value)
                e.target.value = v;
        }, false
    );
    input_p.addEventListener(
        'input', (e) => {
            const v = Math.min(e.target.max, Math.max(e.target.min, e.target.value));
            s_p.value = v;
            if (v !== e.target.value)
                e.target.value = v;
        }, false
    );
    input_r.addEventListener(
        'input', (e) => {
            const v = Math.min(e.target.max, Math.max(e.target.min, e.target.value));
            s_r.value = v;
            if (v !== e.target.value)
                e.target.value = v;
        }, false
    );
    input_h.addEventListener(
        'input', (e) => {
            const v = Math.min(e.target.max, Math.max(e.target.min, e.target.value));
            s_h.value = v;
            if (v !== e.target.value)
                e.target.value = v;
        }, false
    );
})();