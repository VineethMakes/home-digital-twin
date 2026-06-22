import * as THREE from "https://cdn.jsdelivr.net/npm/three@0.165.0/build/three.module.js";
import init, { wasm_demo_snapshot } from "../pkg/home_twin.js";

const canvas = document.querySelector("#scene");
const metrics = document.querySelector("#metrics");
const detail = document.querySelector("#detail");
const alerts = document.querySelector("#alerts");
const playPause = document.querySelector("#playPause");
const step = document.querySelector("#step");

const scene = new THREE.Scene();
scene.background = new THREE.Color(0x101413);

const camera = new THREE.PerspectiveCamera(42, 1, 0.1, 1000);
camera.position.set(26, 58, 68);
camera.lookAt(24, 0, 16);

const renderer = new THREE.WebGLRenderer({ canvas, antialias: true });
renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
renderer.shadowMap.enabled = true;

const raycaster = new THREE.Raycaster();
const pointer = new THREE.Vector2();
const roomMeshes = new Map();
const deviceMeshes = new Map();
const colors = {
  floor: 0x24342d,
  room: 0x2f4d42,
  roomHot: 0x775b3e,
  selected: 0x8bd8bd,
  wall: 0xe5eee8,
  light: 0xffd166,
  climate: 0x61a5ff,
  camera: 0xf77f8f,
  door: 0xd7b98a,
  motion: 0xa78bfa,
  media: 0xf2a65a,
};

let snapshot = null;
let selectedRoomId = "living";
let minute = 0;
let playing = true;
let lastTick = 0;

scene.add(new THREE.HemisphereLight(0xffffff, 0x24342d, 2.5));

const keyLight = new THREE.DirectionalLight(0xffffff, 2.5);
keyLight.position.set(20, 42, 28);
keyLight.castShadow = true;
scene.add(keyLight);

const base = new THREE.Mesh(
  new THREE.BoxGeometry(56, 1, 40),
  new THREE.MeshStandardMaterial({ color: colors.floor, roughness: 0.72 }),
);
base.position.set(24, -0.65, 16);
base.receiveShadow = true;
scene.add(base);

function readingFor(roomId) {
  return snapshot.twin.readings.find((reading) => reading.room_id === roomId);
}

function devicesFor(roomId) {
  return snapshot.twin.devices.filter((device) => device.room_id === roomId);
}

function deviceColor(kind) {
  return colors[kind.toLowerCase()] ?? 0xffffff;
}

function drawHome() {
  for (const mesh of roomMeshes.values()) scene.remove(mesh);
  for (const mesh of deviceMeshes.values()) scene.remove(mesh);
  roomMeshes.clear();
  deviceMeshes.clear();

  for (const room of snapshot.twin.rooms) {
    const reading = readingFor(room.id);
    const active = room.id === selectedRoomId;
    const warm = reading && reading.temperature_f > 72.4;
    const material = new THREE.MeshStandardMaterial({
      color: active ? colors.selected : warm ? colors.roomHot : colors.room,
      roughness: 0.66,
      metalness: 0.02,
    });
    const roomMesh = new THREE.Mesh(
      new THREE.BoxGeometry(room.bounds.size.x, 1.2, room.bounds.size.y),
      material,
    );
    roomMesh.position.set(
      room.bounds.origin.x + room.bounds.size.x / 2,
      0,
      room.bounds.origin.y + room.bounds.size.y / 2,
    );
    roomMesh.castShadow = true;
    roomMesh.receiveShadow = true;
    roomMesh.userData = { type: "room", roomId: room.id };
    scene.add(roomMesh);
    roomMeshes.set(room.id, roomMesh);

    const wallShape = new THREE.EdgesGeometry(roomMesh.geometry);
    const walls = new THREE.LineSegments(
      wallShape,
      new THREE.LineBasicMaterial({ color: colors.wall, transparent: true, opacity: 0.32 }),
    );
    walls.position.copy(roomMesh.position);
    scene.add(walls);

    for (const device of devicesFor(room.id)) {
      const geometry = new THREE.SphereGeometry(device.status === "Off" ? 0.55 : 0.82, 24, 16);
      const deviceMesh = new THREE.Mesh(
        geometry,
        new THREE.MeshStandardMaterial({
          color: deviceColor(device.kind),
          emissive: device.status === "On" ? deviceColor(device.kind) : 0x000000,
          emissiveIntensity: device.status === "On" ? 0.24 : 0,
          roughness: 0.36,
        }),
      );
      deviceMesh.position.set(device.position.x, 2.2, device.position.y);
      deviceMesh.castShadow = true;
      deviceMesh.userData = { type: "device", roomId: room.id, deviceId: device.id };
      scene.add(deviceMesh);
      deviceMeshes.set(device.id, deviceMesh);
    }
  }
}

function renderPanel() {
  const summary = snapshot.summary;
  metrics.innerHTML = [
    ["Rooms", summary.room_count],
    ["Devices", summary.device_count],
    ["Occupied", summary.occupied_rooms],
    ["Load", `${summary.total_energy_watts.toFixed(0)} W`],
  ]
    .map(([label, value]) => `<div class="metric"><b>${value}</b><span>${label}</span></div>`)
    .join("");

  const room = snapshot.twin.rooms.find((item) => item.id === selectedRoomId) ?? snapshot.twin.rooms[0];
  const reading = readingFor(room.id);
  const devices = devicesFor(room.id);
  detail.innerHTML = `
    <h2>${room.name}</h2>
    <dl>
      <dt>Temperature</dt><dd>${reading.temperature_f.toFixed(1)} F</dd>
      <dt>Humidity</dt><dd>${reading.humidity_pct.toFixed(0)}%</dd>
      <dt>Air quality</dt><dd>${reading.air_quality_index} AQI</dd>
      <dt>Occupancy</dt><dd>${reading.occupancy}</dd>
      <dt>Devices</dt><dd>${devices.length}</dd>
    </dl>
    <small>${devices.map((device) => device.name).join(" · ")}</small>
  `;

  alerts.innerHTML = `
    <h2>Automation Watch</h2>
    ${
      summary.alerts.length
        ? summary.alerts
            .map(
              (event) => `
                <div class="alert">
                  <strong>${event.message}</strong>
                  <small>Severity ${event.severity} · ${event.automation_id}</small>
                </div>
              `,
            )
            .join("")
        : '<div class="alert"><strong>All clear</strong><small>No automation issues in this snapshot</small></div>'
    }
  `;
}

function resize() {
  const { clientWidth, clientHeight } = canvas.parentElement;
  renderer.setSize(clientWidth, clientHeight, false);
  camera.aspect = clientWidth / clientHeight;
  camera.updateProjectionMatrix();
}

function refreshSnapshot() {
  snapshot = JSON.parse(wasm_demo_snapshot(minute));
  drawHome();
  renderPanel();
}

function animate(time) {
  requestAnimationFrame(animate);

  if (playing && time - lastTick > 1400) {
    minute += 1;
    refreshSnapshot();
    lastTick = time;
  }

  for (const [id, mesh] of deviceMeshes) {
    const device = snapshot.twin.devices.find((item) => item.id === id);
    mesh.position.y = 2.2 + Math.sin(time * 0.003 + id.length) * (device.status === "On" ? 0.12 : 0.03);
  }

  renderer.render(scene, camera);
}

function handlePick(event) {
  const bounds = canvas.getBoundingClientRect();
  pointer.x = ((event.clientX - bounds.left) / bounds.width) * 2 - 1;
  pointer.y = -((event.clientY - bounds.top) / bounds.height) * 2 + 1;

  raycaster.setFromCamera(pointer, camera);
  const hits = raycaster.intersectObjects([...roomMeshes.values(), ...deviceMeshes.values()]);
  if (hits[0]?.object.userData.roomId) {
    selectedRoomId = hits[0].object.userData.roomId;
    drawHome();
    renderPanel();
  }
}

playPause.addEventListener("click", () => {
  playing = !playing;
  playPause.textContent = playing ? "Pause" : "Play";
  playPause.setAttribute("aria-pressed", String(playing));
});

step.addEventListener("click", () => {
  minute += 1;
  refreshSnapshot();
});

canvas.addEventListener("pointerdown", handlePick);
window.addEventListener("resize", resize);

await init();
resize();
refreshSnapshot();
requestAnimationFrame(animate);
