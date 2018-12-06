const START_X = 500;
const START_Y = 250;
const STEP_SIZE = 18;
const BASE_ANGLE = 0.07;
const BASE_SCALE = 0.9999;      // holiday wreath
// const BASE_SCALE = 0.996;       // spiral
const SUBTREE_ANGLE = -0.4;
const SUBTREE_SCALE = 0.96;
const ZOOM = 1;

// holiday wreath
const BACKGROUND_COLOR = '#f8f0e0';
const BASE_COLOR = '#804818';
const SUBTREE_COLOR = '#008000';
const IGNORED_COLOR = '#ff0000';

// electric weirdness
// const BACKGROUND_COLOR = '#000000';
// const BASE_COLOR = '#ffc0c0';
// const SUBTREE_COLOR = '#c0ffff';
// const IGNORED_COLOR = '#ffffff';

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
        ctx.translate(STEP_SIZE, 0);
        f(node);
        ctx.rotate(theta);
        ctx.scale(scale, scale);
    }
    ctx.restore();
}

function draw_nodes(ctx, nodes, theta, scale, color) {
    // first pass - draw the stem
    ctx.strokeStyle = color;
    ctx.beginPath();
    ctx.moveTo(0,0);
    visit_nodes(ctx, nodes, theta, scale, (node) => {
        ctx.lineTo(0, 0);
        ctx.stroke();
        ctx.beginPath();
        ctx.moveTo(0,0);
    });

    // second pass - recursively draw subtrees
    visit_nodes(ctx, nodes, theta, scale, (node) => {
        if (node.subnodes.length > 0) {
            ctx.save();
            ctx.rotate(SUBTREE_ANGLE);
            draw_nodes(ctx, node.subnodes, -1.5*theta, SUBTREE_SCALE, SUBTREE_COLOR);
            ctx.restore();
        } else if (node.ignored) {
            ctx.fillStyle = IGNORED_COLOR;
            ctx.beginPath();
            ctx.arc(-1,-2,2,0,2*Math.PI);
            ctx.fill();
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
    ctx.fillStyle = BACKGROUND_COLOR;
    ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height);

    ctx.lineWidth = 2;
    ctx.lineCap = 'round';
    ctx.scale(ZOOM, ZOOM);
    ctx.translate(START_X, START_Y);
    draw_nodes(ctx, nodes, BASE_ANGLE, BASE_SCALE, BASE_COLOR);
}

document.addEventListener('DOMContentLoaded', init);
