function forward_line(ctx, dist) {
    ctx.beginPath();
    ctx.moveTo(0,0);
    ctx.translate(dist, 0);
    ctx.lineTo(0,0);
    ctx.stroke();
}

function forward_circle(ctx, dist) {
    ctx.beginPath();
    ctx.arc(dist/2, 0, dist/2, 0, 2*Math.PI);
    ctx.fill();
    ctx.translate(dist, 0);
}

function link_param_control(view, param_name, getControlVal, setControlVal) {
    getControlVal = getControlVal || ((c) => c.value);
    setControlVal = setControlVal || ((c, v) => c.value = v);

    const ctl = document.getElementById(param_name);
    setControlVal(ctl, view[param_name]);
    ctl.addEventListener('input', (event) => {
        view[param_name] = getControlVal(event.target);
        view.draw();
    });
}

function link_checkbox_control(view, param_name) {
    const ctl = document.getElementById(param_name);
    ctl.checked = view[param_name];
    ctl.addEventListener('change', (event) => {
        view[param_name] = ctl.checked;
        view.draw();
    });
}

function simple_file_loader(file_input, ready_callback) {
    const file_picker = document.getElementById(file_input);
    file_picker.addEventListener('change', (event) => {
        if (event.target.files.length > 0) {
            const file = event.target.files[0];
            console.log('reading file', file.name);
            const reader = new FileReader();
            reader.onload = (read_event) => {
                ready_callback(read_event.target.result);
            };
            reader.readAsText(file);
        }
    });
}

function track_mouse(canvas, move_callback) {
    let tracking = null;

    canvas.addEventListener('mousedown', (event) => {
        tracking = { x: event.offsetX, y: event.offsetY };
    });

    canvas.addEventListener('mousemove', (event) => {
        if (tracking) {
            move_callback(event.offsetX - tracking.x, event.offsetY - tracking.y);
            tracking.x = event.offsetX;
            tracking.y = event.offsetY;
        }
    });

    canvas.addEventListener('mouseup', (event) => {
        tracking = null;
    });
}
