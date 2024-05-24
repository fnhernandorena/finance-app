import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

export interface Vehicle {
  id: number;
  brand: string;
  model: string;
  year: number;
  value: number;
}

const vehicles = ref<Vehicle[]>([]);

const loadVehicles = async () => {
  try {
    vehicles.value = await invoke('read_vehicles');
  } catch (e) {
    console.error('Failed to load vehicles', e);
  }
};

const createVehicle = async (brand: string, model: string, year: number, value: number) => {
  try {
    await invoke('create_vehicle', { brand, model, year, value });
    loadVehicles();
  } catch (e) {
    console.error('Failed to create vehicle', e);
  }
};

const updateVehicle = async (id: number, brand: string, model: string, year: number, value: number) => {
  try {
    await invoke('update_vehicle', { id, brand, model, year, value });
    loadVehicles();
  } catch (e) {
    console.error('Failed to update vehicle', e);
  }
};

const deleteVehicle = async (id: number) => {
  try {
    await invoke('delete_vehicle', { id });
    loadVehicles();
  } catch (e) {
    console.error('Failed to delete vehicle', e);
  }
};

export { vehicles, loadVehicles, createVehicle, updateVehicle, deleteVehicle };
