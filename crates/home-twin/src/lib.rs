use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct RoomBounds {
    pub origin: Vec2,
    pub size: Vec2,
}

impl RoomBounds {
    pub fn center(self) -> Vec2 {
        Vec2::new(
            self.origin.x + self.size.x / 2.0,
            self.origin.y + self.size.y / 2.0,
        )
    }

    pub fn area_sq_ft(self) -> f32 {
        self.size.x * self.size.y
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub bounds: RoomBounds,
    pub floor: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeviceKind {
    Light,
    Climate,
    Camera,
    Door,
    Motion,
    Media,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeviceStatus {
    On,
    Off,
    Open,
    Closed,
    Idle,
    Alert,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub room_id: String,
    pub kind: DeviceKind,
    pub status: DeviceStatus,
    pub energy_watts: f32,
    pub position: Vec2,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Reading {
    pub room_id: String,
    pub temperature_f: f32,
    pub humidity_pct: f32,
    pub air_quality_index: u16,
    pub occupancy: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AutomationKind {
    Comfort,
    Security,
    Energy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Automation {
    pub id: String,
    pub name: String,
    pub kind: AutomationKind,
    pub enabled: bool,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AutomationEvent {
    pub automation_id: String,
    pub room_id: String,
    pub severity: u8,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HomeSummary {
    pub room_count: usize,
    pub device_count: usize,
    pub occupied_rooms: usize,
    pub active_devices: usize,
    pub total_energy_watts: f32,
    pub average_temperature_f: f32,
    pub alerts: Vec<AutomationEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HomeTwin {
    pub name: String,
    pub rooms: Vec<Room>,
    pub devices: Vec<Device>,
    pub readings: Vec<Reading>,
    pub automations: Vec<Automation>,
}

#[derive(Debug, Error, PartialEq)]
pub enum TwinError {
    #[error("room '{0}' does not exist")]
    UnknownRoom(String),
    #[error("device '{0}' does not exist")]
    UnknownDevice(String),
    #[error("invalid status '{0}'")]
    InvalidStatus(String),
    #[error("failed to serialize home twin")]
    Serialize,
}

impl HomeTwin {
    pub fn demo() -> Self {
        let rooms = vec![
            Room {
                id: "entry".into(),
                name: "Entry".into(),
                bounds: RoomBounds {
                    origin: Vec2::new(0.0, 0.0),
                    size: Vec2::new(12.0, 10.0),
                },
                floor: 1,
            },
            Room {
                id: "living".into(),
                name: "Living Room".into(),
                bounds: RoomBounds {
                    origin: Vec2::new(12.0, 0.0),
                    size: Vec2::new(22.0, 18.0),
                },
                floor: 1,
            },
            Room {
                id: "kitchen".into(),
                name: "Kitchen".into(),
                bounds: RoomBounds {
                    origin: Vec2::new(12.0, 18.0),
                    size: Vec2::new(16.0, 14.0),
                },
                floor: 1,
            },
            Room {
                id: "office".into(),
                name: "Office".into(),
                bounds: RoomBounds {
                    origin: Vec2::new(34.0, 0.0),
                    size: Vec2::new(14.0, 14.0),
                },
                floor: 1,
            },
            Room {
                id: "bedroom".into(),
                name: "Bedroom".into(),
                bounds: RoomBounds {
                    origin: Vec2::new(28.0, 14.0),
                    size: Vec2::new(20.0, 18.0),
                },
                floor: 1,
            },
        ];

        let devices = vec![
            Device::new(
                "entry-lock",
                "Smart Lock",
                "entry",
                DeviceKind::Door,
                DeviceStatus::Closed,
                2.0,
                Vec2::new(2.0, 5.0),
            ),
            Device::new(
                "living-lights",
                "Scene Lights",
                "living",
                DeviceKind::Light,
                DeviceStatus::On,
                42.0,
                Vec2::new(21.0, 8.0),
            ),
            Device::new(
                "living-motion",
                "Motion Sensor",
                "living",
                DeviceKind::Motion,
                DeviceStatus::On,
                1.0,
                Vec2::new(30.0, 15.0),
            ),
            Device::new(
                "kitchen-climate",
                "Climate Zone",
                "kitchen",
                DeviceKind::Climate,
                DeviceStatus::On,
                650.0,
                Vec2::new(21.0, 25.0),
            ),
            Device::new(
                "office-camera",
                "Office Camera",
                "office",
                DeviceKind::Camera,
                DeviceStatus::Idle,
                5.0,
                Vec2::new(43.0, 3.0),
            ),
            Device::new(
                "bedroom-speaker",
                "Bedroom Speaker",
                "bedroom",
                DeviceKind::Media,
                DeviceStatus::Off,
                0.4,
                Vec2::new(38.0, 25.0),
            ),
        ];

        let readings = vec![
            Reading::new("entry", 68.4, 45.0, 18, 0),
            Reading::new("living", 70.1, 43.0, 22, 2),
            Reading::new("kitchen", 73.5, 48.0, 31, 1),
            Reading::new("office", 69.2, 41.0, 20, 0),
            Reading::new("bedroom", 67.8, 46.0, 16, 0),
        ];

        let automations = vec![
            Automation {
                id: "comfort-balance".into(),
                name: "Comfort Balance".into(),
                kind: AutomationKind::Comfort,
                enabled: true,
                summary: "Watch occupied rooms for temperature drift.".into(),
            },
            Automation {
                id: "away-security".into(),
                name: "Away Security".into(),
                kind: AutomationKind::Security,
                enabled: true,
                summary: "Flag open doors or active cameras while the home is empty.".into(),
            },
            Automation {
                id: "energy-watch".into(),
                name: "Energy Watch".into(),
                kind: AutomationKind::Energy,
                enabled: true,
                summary: "Find rooms using power while nobody is there.".into(),
            },
        ];

        Self {
            name: "Vineeth Makes Home Digital Twin".into(),
            rooms,
            devices,
            readings,
            automations,
        }
    }

    pub fn room(&self, room_id: &str) -> Option<&Room> {
        self.rooms.iter().find(|room| room.id == room_id)
    }

    pub fn device(&self, device_id: &str) -> Option<&Device> {
        self.devices.iter().find(|device| device.id == device_id)
    }

    pub fn set_device_status(
        &mut self,
        device_id: &str,
        status: DeviceStatus,
    ) -> Result<(), TwinError> {
        let device = self
            .devices
            .iter_mut()
            .find(|device| device.id == device_id)
            .ok_or_else(|| TwinError::UnknownDevice(device_id.to_string()))?;

        device.status = status;
        Ok(())
    }

    pub fn update_reading(&mut self, reading: Reading) -> Result<(), TwinError> {
        if self.room(&reading.room_id).is_none() {
            return Err(TwinError::UnknownRoom(reading.room_id));
        }

        match self
            .readings
            .iter_mut()
            .find(|existing| existing.room_id == reading.room_id)
        {
            Some(existing) => *existing = reading,
            None => self.readings.push(reading),
        }

        Ok(())
    }

    pub fn simulate_minute(&mut self, minute: u32) {
        for reading in &mut self.readings {
            let phase = (minute as f32 / 12.0) + reading.room_id.len() as f32;
            reading.temperature_f += phase.sin() * 0.12;
            reading.humidity_pct = (reading.humidity_pct + phase.cos() * 0.05).clamp(35.0, 55.0);

            if reading.room_id == "living" {
                reading.occupancy = if minute % 8 < 5 { 2 } else { 0 };
            }

            if reading.room_id == "office" {
                reading.occupancy = if minute % 15 > 10 { 1 } else { 0 };
            }
        }

        let occupied: BTreeMap<String, u8> = self
            .readings
            .iter()
            .map(|reading| (reading.room_id.clone(), reading.occupancy))
            .collect();

        for device in &mut self.devices {
            if matches!(device.kind, DeviceKind::Light | DeviceKind::Media)
                && occupied.get(&device.room_id).copied().unwrap_or_default() == 0
                && minute.is_multiple_of(10)
            {
                device.status = DeviceStatus::Off;
            }
        }
    }

    pub fn evaluate_automations(&self) -> Vec<AutomationEvent> {
        let mut events = Vec::new();
        let occupied: BTreeMap<&str, u8> = self
            .readings
            .iter()
            .map(|reading| (reading.room_id.as_str(), reading.occupancy))
            .collect();

        for reading in &self.readings {
            if reading.occupancy > 0 && !(68.0..=72.5).contains(&reading.temperature_f) {
                events.push(AutomationEvent {
                    automation_id: "comfort-balance".into(),
                    room_id: reading.room_id.clone(),
                    severity: 2,
                    message: format!(
                        "{} is occupied and at {:.1} F",
                        self.room_name(&reading.room_id),
                        reading.temperature_f
                    ),
                });
            }

            if reading.air_quality_index >= 50 {
                events.push(AutomationEvent {
                    automation_id: "comfort-balance".into(),
                    room_id: reading.room_id.clone(),
                    severity: 1,
                    message: format!(
                        "{} air quality is drifting",
                        self.room_name(&reading.room_id)
                    ),
                });
            }
        }

        for device in &self.devices {
            if occupied
                .get(device.room_id.as_str())
                .copied()
                .unwrap_or_default()
                == 0
                && matches!(device.status, DeviceStatus::On)
                && device.energy_watts > 25.0
            {
                events.push(AutomationEvent {
                    automation_id: "energy-watch".into(),
                    room_id: device.room_id.clone(),
                    severity: 1,
                    message: format!("{} is on in an empty room", device.name),
                });
            }

            if matches!(device.kind, DeviceKind::Door)
                && matches!(device.status, DeviceStatus::Open)
            {
                events.push(AutomationEvent {
                    automation_id: "away-security".into(),
                    room_id: device.room_id.clone(),
                    severity: 3,
                    message: format!("{} is open", device.name),
                });
            }
        }

        events
    }

    pub fn summary(&self) -> HomeSummary {
        let occupied_rooms = self
            .readings
            .iter()
            .filter(|reading| reading.occupancy > 0)
            .count();
        let active_devices = self
            .devices
            .iter()
            .filter(|device| matches!(device.status, DeviceStatus::On | DeviceStatus::Open))
            .count();
        let total_energy_watts = self
            .devices
            .iter()
            .filter(|device| !matches!(device.status, DeviceStatus::Off))
            .map(|device| device.energy_watts)
            .sum();
        let average_temperature_f = self
            .readings
            .iter()
            .map(|reading| reading.temperature_f)
            .sum::<f32>()
            / self.readings.len().max(1) as f32;

        HomeSummary {
            room_count: self.rooms.len(),
            device_count: self.devices.len(),
            occupied_rooms,
            active_devices,
            total_energy_watts,
            average_temperature_f,
            alerts: self.evaluate_automations(),
        }
    }

    pub fn to_json(&self) -> Result<String, TwinError> {
        serde_json::to_string(self).map_err(|_| TwinError::Serialize)
    }

    pub fn snapshot_json(&mut self, minute: u32) -> Result<String, TwinError> {
        self.simulate_minute(minute);
        serde_json::to_string(&Snapshot {
            twin: self.clone(),
            summary: self.summary(),
        })
        .map_err(|_| TwinError::Serialize)
    }

    fn room_name(&self, room_id: &str) -> String {
        self.room(room_id)
            .map(|room| room.name.clone())
            .unwrap_or_else(|| room_id.to_string())
    }
}

impl Device {
    pub fn new(
        id: &str,
        name: &str,
        room_id: &str,
        kind: DeviceKind,
        status: DeviceStatus,
        energy_watts: f32,
        position: Vec2,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            room_id: room_id.into(),
            kind,
            status,
            energy_watts,
            position,
        }
    }
}

impl Reading {
    pub fn new(
        room_id: &str,
        temperature_f: f32,
        humidity_pct: f32,
        air_quality_index: u16,
        occupancy: u8,
    ) -> Self {
        Self {
            room_id: room_id.into(),
            temperature_f,
            humidity_pct,
            air_quality_index,
            occupancy,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Snapshot {
    pub twin: HomeTwin,
    pub summary: HomeSummary,
}

pub fn demo_snapshot_json(minute: u32) -> Result<String, TwinError> {
    HomeTwin::demo().snapshot_json(minute)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn wasm_demo_snapshot(minute: u32) -> Result<String, wasm_bindgen::JsValue> {
    demo_snapshot_json(minute).map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_home_has_valid_room_references() {
        let twin = HomeTwin::demo();

        for device in &twin.devices {
            assert!(twin.room(&device.room_id).is_some());
        }

        for reading in &twin.readings {
            assert!(twin.room(&reading.room_id).is_some());
        }
    }

    #[test]
    fn summary_counts_occupied_rooms_and_energy() {
        let twin = HomeTwin::demo();
        let summary = twin.summary();

        assert_eq!(summary.room_count, 5);
        assert_eq!(summary.device_count, 6);
        assert_eq!(summary.occupied_rooms, 2);
        assert!(summary.total_energy_watts > 600.0);
        assert!(summary.average_temperature_f > 68.0);
    }

    #[test]
    fn open_door_triggers_security_alert() {
        let mut twin = HomeTwin::demo();
        twin.set_device_status("entry-lock", DeviceStatus::Open)
            .unwrap();

        let alerts = twin.evaluate_automations();

        assert!(alerts
            .iter()
            .any(|event| event.automation_id == "away-security" && event.severity == 3));
    }

    #[test]
    fn updating_unknown_room_fails() {
        let mut twin = HomeTwin::demo();
        let result = twin.update_reading(Reading::new("garage", 66.0, 40.0, 12, 0));

        assert_eq!(result, Err(TwinError::UnknownRoom("garage".into())));
    }

    #[test]
    fn snapshot_serializes_for_web() {
        let json = demo_snapshot_json(3).unwrap();

        assert!(json.contains("Vineeth Makes Home Digital Twin"));
        assert!(json.contains("summary"));
    }
}
