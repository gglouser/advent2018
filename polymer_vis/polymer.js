const START_X = 400;
const START_Y = 210;
const STEP_SIZE = 15;
const BASE_ANGLE = 0.07;
const BASE_SCALE = 0.9999;
const SUBTREE_ANGLE = -0.4;
const SUBTREE_SCALE = 0.96;

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

function visit_nodes(ctx, nodes, theta, scale, f) {
    ctx.save();
    for (let node of nodes) {
        f(node);
        ctx.translate(STEP_SIZE, 0);
        ctx.rotate(theta);
        ctx.scale(scale, scale);
    }
    ctx.restore();
}

function draw_nodes(ctx, nodes, theta, scale, color) {
    // first pass - draw the stem
    ctx.beginPath();
    ctx.moveTo(0,0);
    visit_nodes(ctx, nodes, theta, scale, (node) => {
        ctx.lineTo(0, 0);
    });
    ctx.strokeStyle = color;
    ctx.stroke();

    // second pass - recursively draw subtrees
    visit_nodes(ctx, nodes, theta, scale, (node) => {
        if (node.subnodes.length > 0) {
            ctx.save();
            ctx.rotate(SUBTREE_ANGLE);
            draw_nodes(ctx, node.subnodes, -2*theta, SUBTREE_SCALE, "green");
            ctx.restore();
        } else if (node.ignored) {
            ctx.fillStyle = 'red';
            ctx.fillRect(-1,-3,4,4);
        }
    });
}

function init() {
    const nodes = collapse(POLYMER_STRING, IGNORE_UNIT);
    console.debug(nodes.length);

    const canvas = document.getElementById('main');
    canvas.width = canvas.clientWidth;
    canvas.height = canvas.clientHeight;

    const ctx = canvas.getContext('2d');
    // ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    ctx.fillStyle = '#f8f0e0';
    ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height);

    ctx.translate(START_X, START_Y);
    draw_nodes(ctx, nodes, BASE_ANGLE, BASE_SCALE, "#804818");
}

document.addEventListener('DOMContentLoaded', init);
