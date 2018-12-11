// Square
// const START_X = -100;
// const START_Y = 0;
// const ZOOM = 3;
// const STEP_SIZE = 20;
// const START_ANGLE = 0;
// const BASE_ANGLE = 0;
// const BASE_SCALE = 1;
// const BRANCH_ANGLE = 1.57;
// const BRANCH_SCALE = 0.3;
// const METADATA_SCALE = 1;

// Tree #1
const START_X = 0;
const START_Y = 50;
const ZOOM = 4;
const STEP_SIZE = 20;
const START_ANGLE = -1.57;
const BASE_ANGLE = -0.3;
const BASE_SCALE = 0.85;
const BRANCH_ANGLE = 0.4;
const BRANCH_SCALE = 0.8;
const METADATA_SCALE = 1;

// Pine tree
// const BASE_ANGLE = 0;
// const BASE_SCALE = 0.7;
// const BRANCH_ANGLE = 2.14;
// const BRANCH_SCALE = 0.8;
// const METADATA_SCALE = 1;

const METADATA_PART2 = true;
const METADATA_STUBS = false;

const BASE_COLOR = '#808080';
const METADATA_COLOR = '#ff0000';
const BACKGROUND_COLOR = '#f0f0f0';


function parse_license(license) {
    const entries = license.split(' ').map((e) => parseInt(e));
    const get_node = () => {
        const num_child = entries.shift();
        const num_metadata = entries.shift();
        const subnodes = [];
        for (let i = 0; i < num_child; i++) {
            subnodes.push(get_node());
        }
        const metadata = [];
        for (let i = 0; i < num_metadata; i++) {
            metadata.push(entries.shift());
        }
        return { subnodes, metadata };
    };
    return get_node();
}

class TreeView {
    constructor(canvas, node) {
        this.canvas = canvas;
        this.root = node;

        this.start_x = START_X;
        this.start_y = START_Y;
        this.start_angle = START_ANGLE;
        this.zoom = ZOOM;
        this.base_weight = 2;
        this.step_size = STEP_SIZE;
        this.base_angle = BASE_ANGLE;
        this.base_scale = BASE_SCALE;
        this.branch_angle = BRANCH_ANGLE;
        this.branch_scale = BRANCH_SCALE;
        this.metadata_scale = METADATA_SCALE;
        this.metadata_part2 = METADATA_PART2;
        this.metadata_stubs = METADATA_STUBS;

        this.base_color = BASE_COLOR;
        this.metadata_color = METADATA_COLOR;
        this.background_color = BACKGROUND_COLOR;
    }

    draw() {
        const ctx = this.canvas.getContext('2d');
        ctx.fillStyle = this.background_color;
        ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height);

        ctx.lineWidth = this.base_weight;
        ctx.lineCap = 'round';
        ctx.strokeStyle = this.base_color;
        ctx.fillStyle = this.metadata_color;

        ctx.save();
        ctx.translate(ctx.canvas.width/2, ctx.canvas.height/2);
        ctx.scale(this.zoom, this.zoom);
        ctx.translate(this.start_x, this.start_y);
        ctx.rotate(this.start_angle);
        if (this.metadata_part2) {
            this.draw_node_part2(ctx, this.root);
        } else {
            this.draw_node_part1(ctx, this.root);
        }
        ctx.restore();
    }

    draw_node_part2(ctx, node) {
        if (node.subnodes.length > 0) {
            for (let i of node.metadata) {
                if (!this.metadata_stubs && (i < 1 || i > node.subnodes.length)) {
                    continue;
                }

                this.forward_line(ctx, this.step_size);

                ctx.save();
                ctx.rotate(this.branch_angle);
                ctx.scale(this.branch_scale, this.branch_scale);
                if (i >= 1 && i <= node.subnodes.length) {
                    const subn = node.subnodes[i-1];
                    this.draw_node_part2(ctx, subn);
                } else if (this.metadata_stubs) {
                    this.forward_line(ctx, this.step_size/2);
                }
                ctx.restore();

                ctx.rotate(this.base_angle);
                ctx.scale(this.base_scale, this.base_scale);
            }
        } else {
            this.draw_metadata(ctx, node.metadata);
        }
    }

    draw_node_part1(ctx, node) {
        for (let subn of node.subnodes) {
            this.forward_line(ctx, this.step_size);

            ctx.save();
            ctx.rotate(this.branch_angle);
            ctx.scale(this.branch_scale, this.branch_scale);
            this.draw_node_part1(ctx, subn);
            ctx.restore();

            ctx.rotate(this.base_angle);
            ctx.scale(this.base_scale, this.base_scale);
        }

        this.forward_line(ctx, this.step_size);
        this.draw_metadata(ctx, node.metadata);
    }

    draw_metadata(ctx, metadata) {
        ctx.scale(this.metadata_scale, this.metadata_scale);
        for (let v of metadata) {
            this.forward_circle(ctx, v);
        }
    }

    forward_line(ctx, dist) {
        ctx.beginPath();
        ctx.moveTo(0,0);
        ctx.translate(dist, 0);
        ctx.lineTo(0,0);
        ctx.stroke();
    }

    forward_circle(ctx, radius) {
        ctx.beginPath();
        ctx.arc(radius, 0, radius, 0, 2*Math.PI);
        ctx.fill();
        ctx.translate(2*radius, 0);
    }
}

function init() {
    const parse_start = Date.now();
    const root = parse_license('2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2');
    // const root = parse_license(LICENSE);
    const elapsed = Date.now() - parse_start;
    console.log('license tree ready --', elapsed, 'ms');
    // console.debug(root);

    const canvas = document.getElementById('main');
    const view = new TreeView(canvas, root);

    window.requestAnimationFrame(() => {
        canvas.width = canvas.clientWidth;
        canvas.height = canvas.clientHeight;
        view.draw()
    });

    // On window resize, fix canvas size and redraw.
    window.addEventListener('resize', (event) => {
        canvas.width = canvas.clientWidth;
        canvas.height = canvas.clientHeight;
        view.draw();
    });

    link_param_control(view, 'base_weight', (ctl) => parseFloat(ctl.value));
    link_param_control(view, 'step_size', (ctl) => parseFloat(ctl.value));
    link_param_control(view, 'start_angle', (ctl) => parseFloat(ctl.value));
    link_param_control(view, 'base_angle', (ctl) => parseFloat(ctl.value));
    link_param_control(view, 'base_scale', (ctl) => parseFloat(ctl.value));
    link_param_control(view, 'branch_angle', (ctl) => parseFloat(ctl.value));
    link_param_control(view, 'branch_scale', (ctl) => parseFloat(ctl.value));
    link_param_control(view, 'metadata_scale', (ctl) => parseFloat(ctl.value));
    link_checkbox_control(view, 'metadata_part2');
    link_checkbox_control(view, 'metadata_stubs');
    link_param_control(view, 'base_color');
    link_param_control(view, 'metadata_color');
    link_param_control(view, 'background_color');

    simple_file_loader('license_file', (new_lic) => {
        view.root = parse_license(new_lic);
        view.draw();
    });

    track_mouse(canvas, (dx, dy) => {
        view.start_x += dx/view.zoom;
        view.start_y += dy/view.zoom;
        view.draw();
    });

    canvas.addEventListener('wheel', (event) => {
        if (event.deltaY < 0) {
            view.zoom *= 1.25;
        } else {
            view.zoom *= 0.8;
        }
        view.draw();
    }, {passive:true});

}

document.addEventListener('DOMContentLoaded', init);
