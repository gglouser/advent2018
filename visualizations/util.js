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
    link_param_control(view, param_name,
        (ctl) => ctl.checked,
        (ctl, val) => ctl.checked = val);
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
