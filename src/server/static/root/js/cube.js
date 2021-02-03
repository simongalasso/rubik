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
    // var cubesEdges = [];
    for (var x = -1; x < 2; x++) {
        for (var y = -1; y < 2; y++) {
            for (var z = -1; z < 2; z++) {
                var materials = [
                    new THREE.MeshPhongMaterial({color: 0xff7800}),
                    new THREE.MeshPhongMaterial({color: 0xd92b2c}),
                    new THREE.MeshPhongMaterial({color: 0xffffff}),
                    new THREE.MeshPhongMaterial({color: 0xe6e621}),
                    new THREE.MeshPhongMaterial({color: 0x2f55cf}),
                    new THREE.MeshPhongMaterial({color: 0x26b143}),
                ];
                
                var cubeGeometry = new THREE.BoxGeometry(3.95, 3.95, 3.95);
                var cube = new THREE.Mesh(cubeGeometry, materials);
                
                cube.x = x;
                cube.y = y;
                cube.z = z;

                cube.position.x = x * 4;
                cube.position.y = y * 4;
                cube.position.z = z * 4;
                
                // var geometry2 = new THREE.BoxGeometry(3.5, 3.5, 3.5);
                // var edges2 = new THREE.EdgesGeometry(geometry2);
                // var edges = new THREE.LineSegments(edges2, new THREE.LineBasicMaterial( { color: 0x000000 } ));

                // edges.position.x = (x - 1) * 3.6;
                // edges.position.y = (y - 1) * 3.6;
                // edges.position.z = (z - 1) * 3.6;
                
                cubes.push(cube);
                // cubesEdges.push(edges);
            }
        }
    }
    
    cubes.forEach(function(cube) {
        scene.add(cube);
    });
    
    // cubesEdges.forEach(function(cubeEdges) {
    //     scene.add(cubeEdges);
    // });
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