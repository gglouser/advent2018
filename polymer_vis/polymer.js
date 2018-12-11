const START_X = 0;
const START_Y = -150;
const ZOOM = 1;
const STEP_SIZE = 18;
const BASE_ANGLE = 0.07;
const BASE_SCALE = 0.9999;      // holiday wreath
// const BASE_SCALE = 0.996;       // spiral
const BRANCH_ANGLE = -0.6;
const SUBTREE_ANGLE_MULT = -1.5;
const SUBTREE_SCALE = 0.96;

// holiday wreath
const BACKGROUND_COLOR = '#f8f0e0';
const BASE_COLOR = '#804818';
const SUBTREE_COLOR = '#008000';
const IGNORED_COLOR = '#ff0000';

// electric weirdness
// const BACKGROUND_COLOR = '#000000';
// const BASE_COLOR = '#ffc0c0';
// const SUBTREE_COLOR = '#c0ffff';
// const IGNORED_COLOR = '#ffc0f0';

const IGNORE_UNIT = 'c';

function test_react(a, b) {
    return a != b && a.toLowerCase() == b.toLowerCase();
}

function collapse(poly, ignore) {
    const root = { unit: '0', subnodes: [] };
    const nodes = [root];
    for (let u of poly) {
        if (ignore !== undefined && u.toLowerCase() == ignore) {
            nodes[nodes.length-1].subnodes.push({ ignored:true, subnodes:[] });
        } else if (test_react(u, nodes[nodes.length-1].unit)) {
            const last = nodes.pop();
            nodes[nodes.length-1].subnodes.push(last);
        } else {
            nodes.push({ unit: u, subnodes: [] });
        }
    }
    return nodes;
}

class PolymerView {
    constructor(canvas, nodes) {
        this.canvas = canvas;
        this.nodes = nodes;
        
        this.start_x = START_X;
        this.start_y = START_Y;
        this.zoom = ZOOM;
        this.base_weight = 2;
        this.step_size = STEP_SIZE;
        this.base_angle = BASE_ANGLE;
        this.base_scale = BASE_SCALE;
        this.branch_angle = BRANCH_ANGLE;
        this.subtree_angle_mult = SUBTREE_ANGLE_MULT;
        this.subtree_scale = SUBTREE_SCALE;

        this.background_color = BACKGROUND_COLOR;
        this.base_color = BASE_COLOR;
        this.subtree_color = SUBTREE_COLOR;
        this.ignored_color = IGNORED_COLOR;
    }

    draw() {
        const ctx = this.canvas.getContext('2d');
        ctx.fillStyle = this.background_color;
        ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height);

        ctx.lineWidth = this.base_weight;
        ctx.lineCap = 'round';
        ctx.save();
        ctx.translate(ctx.canvas.width/2, ctx.canvas.height/2);
        ctx.scale(this.zoom, this.zoom);
        ctx.translate(this.start_x, this.start_y);
        this.draw_nodes(ctx, this.nodes, this.base_angle, this.base_scale, this.base_color);
        ctx.restore();
    }

    visit_nodes(ctx, nodes, theta, scale, f) {
        // nodes = nodes.slice(0).reverse();
        ctx.save();
        for (let node of nodes) {
            ctx.translate(this.step_size, 0);
            f(node);
            ctx.rotate(theta);
            ctx.scale(scale, scale);
        }
        ctx.restore();
    }

    draw_nodes(ctx, nodes, theta, scale, color) {
        // first pass - draw the stem
        ctx.strokeStyle = color;
        ctx.beginPath();
        ctx.moveTo(0,0);
        this.visit_nodes(ctx, nodes, theta, scale, (node) => {
            ctx.lineTo(0, 0);
            ctx.stroke();
            ctx.beginPath();
            ctx.moveTo(0,0);
        });

        // second pass - recursively draw subtrees
        this.visit_nodes(ctx, nodes, theta, scale, (node) => {
            if (node.subnodes.length > 0) {
                ctx.save();
                ctx.rotate(this.branch_angle);
                this.draw_nodes(ctx, node.subnodes,
                    this.subtree_angle_mult*theta,
                    this.subtree_scale,
                    this.subtree_color);
                ctx.restore();
            } else if (node.ignored) {
                ctx.fillStyle = this.ignored_color;
                ctx.beginPath();
                ctx.arc(-1,-2,3,0,2*Math.PI);
                ctx.fill();
            }
        });
    }
}

function animate(view, n, dn, ddn) {
    view.nodes = collapse(POLYMER_STRING.substring(0, n), IGNORE_UNIT);
    view.draw();
    if (Math.floor(n/1000) != Math.floor((n-dn)/1000)) {
        console.debug('...', Math.floor(n/1000)*1000);
    }
    if (n < POLYMER_STRING.length) {
        window.requestAnimationFrame(() => animate(view, n+dn, dn+ddn, ddn));
    }
}

function init() {
    const collapse_start = Date.now();
    const nodes = collapse(POLYMER_STRING, IGNORE_UNIT);
    const elapsed = Date.now() - collapse_start;
    console.log('polymer ready --', nodes.length, 'nodes --', elapsed, 'ms');

    const canvas = document.getElementById('main');
    canvas.width = canvas.clientWidth;
    canvas.height = canvas.clientHeight;

    window.requestAnimationFrame(() => {
        canvas.width = canvas.clientWidth;
        canvas.height = canvas.clientHeight;
        view.draw();
        // animate(view, 1, 8, 0);
    });
    
    // On window resize, fix canvas size and redraw.
    window.addEventListener('resize', (event) => {
        canvas.width = canvas.clientWidth;
        canvas.height = canvas.clientHeight;
        view.draw();
    });

    const view = new PolymerView(canvas, nodes);
    link_param_control(view, 'base_weight', (ctl) => parseFloat(ctl.value));
    link_param_control(view, 'step_size', (ctl) => parseFloat(ctl.value));
    link_param_control(view, 'base_angle', (ctl) => parseFloat(ctl.value));
    link_param_control(view, 'base_scale', (ctl) => parseFloat(ctl.value));
    link_param_control(view, 'branch_angle', (ctl) => parseFloat(ctl.value));
    link_param_control(view, 'subtree_angle_mult', (ctl) => parseFloat(ctl.value));
    link_param_control(view, 'subtree_scale', (ctl) => parseFloat(ctl.value));
    link_param_control(view, 'base_color');
    link_param_control(view, 'subtree_color');
    link_param_control(view, 'ignored_color');
    link_param_control(view, 'background_color');

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
