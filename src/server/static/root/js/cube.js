const ORANGE = 0xff7800;
const RED = 0xd92b2c;
const WHITE = 0xffffff;
const BLACK = 0x000000;
const DARK_GREY = 0x161616;
const GREEN = 0x26b143;
const BLUE = 0x2f55cf;
const YELLOW = 0xe6e621;

// QUEUE
var queue = []

const enqueue = (element) => {
    queue.push(element);
}

const dequeue = () => { 
    if (queue.length == 0) 
        console.log("Queue is empty!")
    else
        return queue.shift(); 
}

const front = () => {
    if (queue.length == 0) 
        console.log("No item in queue!")
    else
        return queue[0];
}


var pivot = new THREE.Object3D();
var moving = false;
var speed = 0.02;
var cubes = [];
var action = {};

function createCubes(scene) {
    var cubes = [];
    for (var x = -1; x < 2; x++) {
        for (var y = -1; y < 2; y++) {
            for (var z = -1; z < 2; z++) {
                var materials = [];
                (x == 1) ? materials.push(new THREE.MeshPhongMaterial({color: ORANGE})) : materials.push(new THREE.MeshPhongMaterial({color: DARK_GREY}));
                (x == -1) ? materials.push(new THREE.MeshPhongMaterial({color: RED})) : materials.push(new THREE.MeshPhongMaterial({color: DARK_GREY}));
                (y == 1) ? materials.push(new THREE.MeshPhongMaterial({color: BLUE})) : materials.push(new THREE.MeshPhongMaterial({color: DARK_GREY}));
                (y == -1) ? materials.push(new THREE.MeshPhongMaterial({color: GREEN})) : materials.push(new THREE.MeshPhongMaterial({color: DARK_GREY}));
                (z == 1) ? materials.push(new THREE.MeshPhongMaterial({color: YELLOW})) : materials.push(new THREE.MeshPhongMaterial({color: DARK_GREY}));
                (z == -1) ? materials.push(new THREE.MeshPhongMaterial({color: WHITE})) : materials.push(new THREE.MeshPhongMaterial({color: DARK_GREY}));

                var cubeGeometry = new THREE.BoxGeometry(3.8, 3.8, 3.8);
                var cube = new THREE.Mesh(cubeGeometry, materials);
                var edgesGeometry = new THREE.EdgesGeometry(cubeGeometry);
                var edges = new THREE.LineSegments(edgesGeometry, new THREE.LineBasicMaterial({color: 0x000000,
                    linewidth: 1.5 }));
                const group = new THREE.Group();
                group.add(cube);
                // group.add(edges);

                group.x = x;
                group.y = y;
                group.z = z;
                
                group.position.x = x * 4;
                group.position.y = y * 4;
                group.position.z = z * 4;

                cubes.push(group);
            }
        }
    }
    
    cubes.forEach(function(cube) {
        scene.add(cube);
    });
    
    return (cubes)
}

const applySequence = (sequence) => {
    const moves = sequence.split(" ");
    moves.map((letter) => {
        enqueue(letter);
    })
}

var scene = new THREE.Scene();
var camera = new THREE.PerspectiveCamera(45, window.innerWidth / window.innerHeight, 1, 1000);
var renderer = new THREE.WebGLRenderer();
var clock = new THREE.Clock();

renderer.setClearColor(new THREE.Color(0xEEEEEE));
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

const light = new THREE.AmbientLight(0xFFFFFF, 0.8);
scene.add(light);

// var group = new THREE.Group();
// scene.add(group);

cubes = createCubes(scene);

var controls = new THREE.OrbitControls(camera, renderer.domElement);
// controls.addEventListener( 'change', render );
controls.target.set(0, 0, 0);
controls.enableKeys = false;
controls.enablePan = false;
controls.minDistance = 15;
controls.maxDistance = 150;
controls.update()

camera.position.set(-30, 30, 50);
controls.update();

const sequence = "R D U L B D F";
applySequence(sequence);

const nextmove = () => {
    if (queue.length == 0) {
        return ;
    } else {
        setCubes(front());
        selectPivot();
        moving = true;
    }
}

nextmove();

function setCubes(face) {
    switch (face) {
        case 'U':
            action = {
                selectedCubes: cubes.filter(cube => cube.y == 1),
                axis: "y",
                direction: -1
            }
            break;
        case 'R':
            action = {
                selectedCubes: cubes.filter(cube => cube.x == 1),
                axis: "x",
                direction: 1
            }
        case 'F':
            action = {
                selectedCubes: cubes.filter(cube => cube.z == -1),
                axis: "z",
                direction: 1
            }
            break;
        case 'D':
            action = {
                selectedCubes: cubes.filter(cube => cube.y == -1),
                axis: "y",
                direction: 1
            }
            break;
        case 'L':
            action = { 
                selectedCubes: cubes.filter(cube => cube.x == 1),
                axis: "x",
                direction: -1
            }
            break;
        case 'B':
            action = {
                selectedCubes: cubes.filter(cube => cube.z == 1),
                axis: "z",
                direction: -1
            }
            break;
    }
}

function selectPivot() {
    pivot.rotation.set(0, 0, 0);
    pivot.updateMatrixWorld();
    for (var i in action.selectedCubes) {
        pivot.attach(action.selectedCubes[i]);
    }
    scene.add(pivot);
}


function move() {
    if (pivot.rotation[action.axis] >= Math.PI / 2) {
       pivot.rotation[action.axis] = Math.PI / 2;
       stop();
    } else if (pivot.rotation[action.axis] <= Math.PI / -2) {
       pivot.rotation[action.axis] = Math.PI / -2;
       stop();
    } else {
       pivot.rotation[action.axis] += speed * action.direction;
    }
}

function stop() {
    moving = false;
    pivot.updateMatrixWorld();
    scene.remove(pivot);
    for (var i in action.selectedCubes) {
        action.selectedCubes[i].updateMatrixWorld();
        scene.attach(action.selectedCubes[i]);
        action.selectedCubes[i].x = Math.round(action.selectedCubes[i].position.x / 4);
        action.selectedCubes[i].y = Math.round(action.selectedCubes[i].position.y / 4);
        action.selectedCubes[i].z = Math.round(action.selectedCubes[i].position.z / 4);
    }
    pivot = new THREE.Object3D();
    dequeue();
    action = {};
    nextmove();
}

var render = function() {
    requestAnimationFrame(render);
    // console.log(scene.position)
    if (moving)
        move();
    renderer.render(scene, camera);
};

render();